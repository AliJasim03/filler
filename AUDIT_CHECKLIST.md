# Filler Project - Audit Checklist

## Docker Setup Commands

### Build the Docker Image
```bash
cd docker_image
docker build -t filler .
```

### Run the Container (Interactive Mode)
```bash
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

### Run Commands Inside Container (One-off)
```bash
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "COMMAND_HERE"
```

---

## Audit Questions & Test Commands

### FUNCTIONAL TESTS

#### Question 1: Can you confirm that the student was able to create the image and container correctly?

**Test Command:**
```bash
# From host machine, in the docker_image directory
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator -q"
```

**Expected Result:** Game runs successfully and shows winner

---

#### Question 2: Can you confirm that the project runs correctly?

**Test Commands:**
```bash
# Build the player inside container
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "cd solution && cargo build --release"

# Test the player against a robot
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map00 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/wall_e -q"
```

**Expected Result:** Player compiles successfully and plays the game

---

#### Question 3: Can you confirm that the student player is placing the pieces correctly with the overlapping of just one cell?

**Code Verification:**
Check `solution/src/main.rs` lines 146-177, specifically line 176:
```rust
overlap_count == 1  // Must overlap exactly once
```

**Test Command:**
```bash
# Run a game and verify it completes without errors
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map00 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/wall_e"
```

**Expected Result:** Game runs without placement errors, player makes valid moves

---

#### Question 4: Can you confirm that the student player won at least 4 out of 5 times against wall_e on map00?

**Test Commands (Run 5 times alternating positions):**

```bash
# Game 1 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map00 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/wall_e -q -s 1"

# Game 2 - Player as p2
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map00 -p1 linux_robots/wall_e -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s 2"

# Game 3 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map00 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/wall_e -q -s 3"

# Game 4 - Player as p2
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map00 -p1 linux_robots/wall_e -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s 4"

# Game 5 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map00 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/wall_e -q -s 5"
```

**Expected Result:** Student player wins at least 4 out of 5 games
**Actual Result:** ✅ **5/5 wins**

---

#### Question 5: Can you confirm that the student player won at least 4 out of 5 times against h2_d2 on map01?

**Test Commands (Run 5 times alternating positions):**

```bash
# Game 1 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/h2_d2 -q -s 1"

# Game 2 - Player as p2
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 linux_robots/h2_d2 -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s 2"

# Game 3 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/h2_d2 -q -s 3"

# Game 4 - Player as p2
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 linux_robots/h2_d2 -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s 4"

# Game 5 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/h2_d2 -q -s 5"
```

**Expected Result:** Student player wins at least 4 out of 5 games
**Actual Result:** ✅ **5/5 wins**

---

#### Question 6: Can you confirm that the student player won at least 4 out of 5 times against bender on map02?

**Test Commands (Run 5 times alternating positions):**

```bash
# Game 1 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map02 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/bender -q -s 1"

# Game 2 - Player as p2
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map02 -p1 linux_robots/bender -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s 2"

# Game 3 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map02 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/bender -q -s 3"

# Game 4 - Player as p2
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map02 -p1 linux_robots/bender -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s 4"

# Game 5 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map02 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/bender -q -s 5"
```

**Expected Result:** Student player wins at least 4 out of 5 games
**Actual Result:** ✅ **5/5 wins**

---

### BASIC REQUIREMENTS

#### Question 7: Does the code obey the good practices?

**Verification Steps:**
1. Check code structure in `solution/src/main.rs`
2. Verify proper error handling
3. Check for clean code organization
4. Verify no security vulnerabilities

**Assessment:**
- ✅ Clean, well-structured Rust code
- ✅ Proper use of types and error handling
- ✅ Good separation of concerns
- ⚠️ Minor: Debug statements present (not critical)

---

#### Question 8: Is there a test file for this code?

**Check Command:**
```bash
# Check for test files
ls -la solution/
cat solution/src/main.rs | grep -E "(#\[test\]|#\[cfg\(test\)\]|mod tests)"
```

**Assessment:**
- ⚠️ `test_input.txt` exists for manual testing
- ❌ No Rust unit tests found

---

#### Question 9: Are the tests checking each possible case?

**Assessment:**
- ❌ N/A - No automated unit tests exist
- ✅ However, functional tests prove correctness (100% win rate)

---

### BONUS REQUIREMENTS

#### Question 10: Did the student create a visualizer for the project?

**Check Command:**
```bash
ls -la solution/ | grep -iE "(visual|gui|display)"
```

**Assessment:** ❌ No visualizer created

---

#### Question 11: Can you confirm that the student player won at least 4 out of 5 times against terminator?

**Test Commands (Run 5 times alternating positions):**

```bash
# Game 1 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/terminator -q -s 1"

# Game 2 - Player as p2
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 linux_robots/terminator -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s 2"

# Game 3 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/terminator -q -s 3"

# Game 4 - Player as p2
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 linux_robots/terminator -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s 4"

# Game 5 - Player as p1
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/terminator -q -s 5"
```

**Expected Result:** Student player wins at least 4 out of 5 games
**Actual Result:** ⚠️ **2/5 wins** (Does not pass bonus requirement)

---

## Quick Test Script

Save this as `run_all_tests.sh` in the `docker_image` directory:

```bash
#!/bin/bash

echo "=========================================="
echo "FILLER PROJECT - COMPLETE AUDIT TEST SUITE"
echo "=========================================="

# Build player
echo ""
echo "[1/4] Building player..."
docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "cd solution && cargo build --release" > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# Test wall_e
echo ""
echo "[2/4] Testing vs wall_e on map00 (5 games)..."
wins=0
for i in 1 2 3 4 5; do
    if [ $((i % 2)) -eq 1 ]; then
        result=$(docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map00 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/wall_e -q -s $i" 2>&1)
        if echo "$result" | grep -q "Player1 won"; then wins=$((wins+1)); fi
    else
        result=$(docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map00 -p1 linux_robots/wall_e -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s $i" 2>&1)
        if echo "$result" | grep -q "Player2 won"; then wins=$((wins+1)); fi
    fi
done
echo "   Wins: $wins/5 (need 4/5)"
[ $wins -ge 4 ] && echo "   ✅ PASS" || echo "   ❌ FAIL"

# Test h2_d2
echo ""
echo "[3/4] Testing vs h2_d2 on map01 (5 games)..."
wins=0
for i in 1 2 3 4 5; do
    if [ $((i % 2)) -eq 1 ]; then
        result=$(docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/h2_d2 -q -s $i" 2>&1)
        if echo "$result" | grep -q "Player1 won"; then wins=$((wins+1)); fi
    else
        result=$(docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map01 -p1 linux_robots/h2_d2 -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s $i" 2>&1)
        if echo "$result" | grep -q "Player2 won"; then wins=$((wins+1)); fi
    fi
done
echo "   Wins: $wins/5 (need 4/5)"
[ $wins -ge 4 ] && echo "   ✅ PASS" || echo "   ❌ FAIL"

# Test bender
echo ""
echo "[4/4] Testing vs bender on map02 (5 games)..."
wins=0
for i in 1 2 3 4 5; do
    if [ $((i % 2)) -eq 1 ]; then
        result=$(docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map02 -p1 solution/target/x86_64-unknown-linux-musl/release/player -p2 linux_robots/bender -q -s $i" 2>&1)
        if echo "$result" | grep -q "Player1 won"; then wins=$((wins+1)); fi
    else
        result=$(docker run --rm -v "$(pwd)/solution":/filler/solution filler -c "./linux_game_engine -f maps/map02 -p1 linux_robots/bender -p2 solution/target/x86_64-unknown-linux-musl/release/player -q -s $i" 2>&1)
        if echo "$result" | grep -q "Player2 won"; then wins=$((wins+1)); fi
    fi
done
echo "   Wins: $wins/5 (need 4/5)"
[ $wins -ge 4 ] && echo "   ✅ PASS" || echo "   ❌ FAIL"

echo ""
echo "=========================================="
echo "AUDIT COMPLETE"
echo "=========================================="
```

Make it executable:
```bash
chmod +x run_all_tests.sh
./run_all_tests.sh
```

---

## Summary

### ✅ PASSING
- Docker image and container setup
- Player runs correctly
- Correct piece placement (exactly 1 cell overlap)
- **5/5 wins** vs wall_e on map00
- **5/5 wins** vs h2_d2 on map01
- **5/5 wins** vs bender on map02
- Code follows good practices

### ⚠️ NOT PASSING
- No unit test files (only manual test input)
- No visualizer (bonus)
- Does not beat terminator 4/5 times (bonus)
