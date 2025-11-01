use std::io;

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct GameState {
    player_num: u8,
    board_height: usize,
    board_width: usize,
    board: Vec<Vec<char>>,
    piece_height: usize,
    piece_width: usize,
    piece: Vec<Vec<char>>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            player_num: 0,
            board_height: 0,
            board_width: 0,
            board: Vec::new(),
            piece_height: 0,
            piece_width: 0,
            piece: Vec::new(),
        }
    }

    fn read_input(&mut self) -> io::Result<bool> {
        let mut line = String::new();

        // Read player number if first turn
        if self.player_num == 0 {
            line.clear();
            if io::stdin().read_line(&mut line)? == 0 {
                eprintln!("DEBUG: EOF reading player number");
                return Ok(false);
            }
            eprintln!("DEBUG: Player line: {}", line.trim());
            if line.contains("p1") {
                self.player_num = 1;
            } else if line.contains("p2") {
                self.player_num = 2;
            } else {
                eprintln!("DEBUG: Failed to parse player number");
                return Ok(false);
            }
            eprintln!("DEBUG: Player number set to {}", self.player_num);
        }

        // Read board
        line.clear();
        if io::stdin().read_line(&mut line)? == 0 {
            eprintln!("DEBUG: EOF reading board header");
            return Ok(false);
        }
        eprintln!("DEBUG: Board line: {}", line.trim());
        if line.starts_with("Plateau") || line.starts_with("Anfield") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                // Format is "Anfield WIDTH HEIGHT:" or "Plateau WIDTH HEIGHT:"
                self.board_width = parts[1].parse().unwrap_or(0);
                self.board_height = parts[2].trim_end_matches(':').parse().unwrap_or(0);
            }
        } else {
            return Ok(false);
        }

        // Skip column numbers line
        line.clear();
        if io::stdin().read_line(&mut line)? == 0 {
            return Ok(false);
        }

        // Read board rows
        self.board.clear();
        for _ in 0..self.board_height {
            line.clear();
            if io::stdin().read_line(&mut line)? == 0 {
                return Ok(false);
            }
            let row: Vec<char> = line.chars().skip(4).take(self.board_width).collect();
            self.board.push(row);
        }

        // Read piece
        line.clear();
        if io::stdin().read_line(&mut line)? == 0 {
            return Ok(false);
        }
        if line.starts_with("Piece") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                // Format is "Piece WIDTH HEIGHT:" - parts[1] is width, parts[2] is height
                self.piece_width = parts[1].parse().unwrap_or(0);
                self.piece_height = parts[2].trim_end_matches(':').parse().unwrap_or(0);
            }
        } else {
            return Ok(false);
        }

        // Read piece rows
        self.piece.clear();
        for _ in 0..self.piece_height {
            line.clear();
            if io::stdin().read_line(&mut line)? == 0 {
                return Ok(false);
            }
            let row: Vec<char> = line.trim_end().chars().take(self.piece_width).collect();
            self.piece.push(row);
        }

        Ok(true)
    }

    fn my_char(&self) -> char {
        // Lowercase: last placed piece
        if self.player_num == 1 { 'a' } else { 's' }
    }

    fn my_territory_char(&self) -> char {
        // Uppercase: older territory
        if self.player_num == 1 { '@' } else { '$' }
    }

    fn opponent_char(&self) -> char {
        if self.player_num == 1 { 's' } else { 'a' }
    }

    fn opponent_territory_char(&self) -> char {
        if self.player_num == 1 { '$' } else { '@' }
    }

    fn is_my_cell(&self, c: char) -> bool {
        c == self.my_char() || c == self.my_territory_char()
    }

    fn is_opponent_cell(&self, c: char) -> bool {
        c == self.opponent_char() || c == self.opponent_territory_char()
    }

    fn can_place_piece(&self, board_y: i32, board_x: i32) -> bool {
        let mut overlap_count = 0;

        for piece_y in 0..self.piece_height {
            for piece_x in 0..self.piece_width {
                if self.piece[piece_y][piece_x] == 'O' {
                    let by = board_y + piece_y as i32;
                    let bx = board_x + piece_x as i32;

                    // Check bounds
                    if by < 0 || by >= self.board_height as i32 || bx < 0 || bx >= self.board_width as i32 {
                        return false;
                    }

                    let cell = self.board[by as usize][bx as usize];

                    // Check if overlapping with opponent
                    if self.is_opponent_cell(cell) {
                        return false;
                    }

                    // Count overlaps with our territory
                    if self.is_my_cell(cell) {
                        overlap_count += 1;
                    }
                }
            }
        }

        // Must overlap exactly once with our territory
        overlap_count == 1
    }

    fn score_position(&self, board_y: i32, board_x: i32) -> i32 {
        let mut score = 0;
        let mut piece_cells = 0;
        let mut min_dist_to_opponent = i32::MAX;

        for piece_y in 0..self.piece_height {
            for piece_x in 0..self.piece_width {
                if self.piece[piece_y][piece_x] == 'O' {
                    let by = board_y + piece_y as i32;
                    let bx = board_x + piece_x as i32;

                    piece_cells += 1;

                    // Count adjacent cells
                    let mut adjacent_opponent = 0;
                    let mut adjacent_empty = 0;
                    let mut adjacent_my_territory = 0;

                    for (dy, dx) in &[(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                        let ny = by + dy;
                        let nx = bx + dx;

                        if ny >= 0 && ny < self.board_height as i32 && nx >= 0 && nx < self.board_width as i32 {
                            let neighbor = self.board[ny as usize][nx as usize];
                            if self.is_opponent_cell(neighbor) {
                                adjacent_opponent += 1;
                            } else if neighbor == '.' {
                                adjacent_empty += 1;
                            } else if self.is_my_cell(neighbor) {
                                adjacent_my_territory += 1;
                            }
                        }
                    }

                    // Find closest opponent cell
                    for oy in 0..self.board_height {
                        for ox in 0..self.board_width {
                            if self.is_opponent_cell(self.board[oy][ox]) {
                                let dist = (by - oy as i32).abs() + (bx - ox as i32).abs();
                                min_dist_to_opponent = min_dist_to_opponent.min(dist);
                            }
                        }
                    }

                    // Scoring strategy:
                    // 1. Aggressively move toward opponent (most important)
                    score -= min_dist_to_opponent * 50;

                    // 2. Reward blocking opponent
                    score += adjacent_opponent * 100;

                    // 3. Reward expanding territory
                    score += adjacent_empty * 10;

                    // 4. Avoid clustering too much
                    score -= adjacent_my_territory * 5;
                }
            }
        }

        // Prefer larger pieces when scores are equal
        score += piece_cells * 2;

        score
    }

    fn find_best_move(&self) -> Position {
        let mut best_score = i32::MIN;
        let mut best_pos = Position { x: 0, y: 0 };
        let mut found_valid_move = false;

        for board_y in 0..self.board_height as i32 {
            for board_x in 0..self.board_width as i32 {
                if self.can_place_piece(board_y, board_x) {
                    found_valid_move = true;
                    let score = self.score_position(board_y, board_x);
                    if score > best_score {
                        best_score = score;
                        best_pos = Position {
                            x: board_x as usize,
                            y: board_y as usize,
                        };
                    }
                }
            }
        }

        if !found_valid_move {
            eprintln!("No valid move found!");
        }

        best_pos
    }
}

fn main() {
    use std::io::Write;
    let mut game = GameState::new();

    loop {
        match game.read_input() {
            Ok(true) => {
                let best_move = game.find_best_move();
                println!("{} {}", best_move.x, best_move.y);
                io::stdout().flush().unwrap();
            }
            Ok(false) => {
                // End of input or error
                println!("0 0");
                io::stdout().flush().unwrap();
                break;
            }
            Err(_) => {
                // Error reading input
                println!("0 0");
                io::stdout().flush().unwrap();
                break;
            }
        }
    }
}
