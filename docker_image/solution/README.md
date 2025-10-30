# Filler Rust Solution

This is a Rust implementation of the Filler game AI that follows the audit requirements.

## Building and Testing

### 1. Build the Docker Image

```bash
cd docker_image
docker build -t filler .
```

### 2. Run the Container

```bash
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

### 3. Compile the Rust Solution

Inside the container:

```bash
cd solution
cargo build --release
```

The compiled binary will be at: `target/release/player`

### 4. Test Against Robots

Run these commands inside the container to test your AI:

#### Test against wall_e (map00)

```bash
# Player 1
./linux_game_engine -f maps/map00 -p1 solution/target/release/player -p2 linux_robots/wall_e

# Player 2
./linux_game_engine -f maps/map00 -p1 linux_robots/wall_e -p2 solution/target/release/player
```

Run this 5 times alternating positions. You should win at least 4/5 times.

#### Test against h2_d2 (map01)

```bash
# Player 1
./linux_game_engine -f maps/map01 -p1 solution/target/release/player -p2 linux_robots/h2_d2

# Player 2
./linux_game_engine -f maps/map01 -p1 linux_robots/h2_d2 -p2 solution/target/release/player
```

Run this 5 times alternating positions. You should win at least 4/5 times.

#### Test against bender (map02)

```bash
# Player 1
./linux_game_engine -f maps/map02 -p1 solution/target/release/player -p2 linux_robots/bender

# Player 2
./linux_game_engine -f maps/map02 -p1 linux_robots/bender -p2 solution/target/release/player
```

Run this 5 times alternating positions. You should win at least 4/5 times.

## Algorithm Overview

The Rust implementation uses a heuristic-based algorithm that:

1. Parses the game state from stdin (board and piece)
2. Checks all possible positions where the piece can be placed
3. Validates that exactly one cell overlaps with existing territory
4. Scores each valid position based on:
   - Proximity to opponent (for blocking)
   - Expansion into empty spaces
   - Strategic positioning
5. Outputs the best move as "X Y\n"

## Architecture

- `src/main.rs`: Main game logic
  - Input parsing (player number, board, pieces)
  - Placement validation
  - Scoring algorithm
  - Move selection

The solution follows Rust best practices and should pass the audit requirements.
