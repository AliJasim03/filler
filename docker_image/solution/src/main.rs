use std::io::{self, BufRead};

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
                return Ok(false);
            }
            if line.contains("p1") {
                self.player_num = 1;
            } else if line.contains("p2") {
                self.player_num = 2;
            } else {
                return Ok(false);
            }
        }

        // Read board
        line.clear();
        if io::stdin().read_line(&mut line)? == 0 {
            return Ok(false);
        }
        if line.starts_with("Plateau") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                self.board_height = parts[1].parse().unwrap_or(0);
                self.board_width = parts[2].trim_end_matches(':').parse().unwrap_or(0);
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
                self.piece_height = parts[1].parse().unwrap_or(0);
                self.piece_width = parts[2].trim_end_matches(':').parse().unwrap_or(0);
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
            let row: Vec<char> = line.chars().take(self.piece_width).collect();
            self.piece.push(row);
        }

        Ok(true)
    }

    fn my_char(&self) -> char {
        if self.player_num == 1 { 'a' } else { 's' }
    }

    fn my_territory_char(&self) -> char {
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
                if self.piece[piece_y][piece_x] == '*' {
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

        // Calculate center of mass for preference
        let center_y = self.board_height / 2;
        let center_x = self.board_width / 2;

        for piece_y in 0..self.piece_height {
            for piece_x in 0..self.piece_width {
                if self.piece[piece_y][piece_x] == '*' {
                    let by = (board_y + piece_y as i32) as usize;
                    let bx = (board_x + piece_x as i32) as usize;

                    // Prefer blocking opponent
                    let mut adjacent_opponent = 0;
                    let mut adjacent_empty = 0;

                    for (dy, dx) in &[(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                        let ny = by as i32 + dy;
                        let nx = bx as i32 + dx;

                        if ny >= 0 && ny < self.board_height as i32 && nx >= 0 && nx < self.board_width as i32 {
                            let neighbor = self.board[ny as usize][nx as usize];
                            if self.is_opponent_cell(neighbor) {
                                adjacent_opponent += 1;
                            } else if neighbor == '.' {
                                adjacent_empty += 1;
                            }
                        }
                    }

                    // Reward being near opponent (for blocking)
                    score += adjacent_opponent * 10;

                    // Reward expansion into empty spaces
                    score += adjacent_empty * 3;

                    // Slight preference for center positions
                    let dist_from_center = (by as i32 - center_y as i32).abs() + (bx as i32 - center_x as i32).abs();
                    score -= dist_from_center / 2;
                }
            }
        }

        score
    }

    fn find_best_move(&self) -> Position {
        let mut best_pos = Position { x: 0, y: 0 };
        let mut best_score = i32::MIN;
        let mut found_valid = false;

        for board_y in -(self.piece_height as i32)..=(self.board_height as i32) {
            for board_x in -(self.piece_width as i32)..=(self.board_width as i32) {
                if self.can_place_piece(board_y, board_x) {
                    let score = self.score_position(board_y, board_x);
                    if !found_valid || score > best_score {
                        best_score = score;
                        best_pos = Position {
                            x: board_x as usize,
                            y: board_y as usize,
                        };
                        found_valid = true;
                    }
                }
            }
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
                println!("{} {}", best_move.y, best_move.x);
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
