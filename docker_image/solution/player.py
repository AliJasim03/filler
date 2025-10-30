#!/usr/bin/env python3
"""
Filler player implementation
Strategy: Find valid placements and choose the best one based on scoring
"""

import sys
import math


def parse_input():
    """Parse the input from game engine"""
    try:
        # First line: player info
        line = input().strip()
        if line.startswith("$$$"):
            player_num = int(line.split()[2][1])
        else:
            return None, None, None

        # Anfield header
        line = input().strip()
        if not line.startswith("Anfield"):
            return None, None, None

        parts = line.split()
        rows = int(parts[1].replace(":", ""))
        cols = int(parts[2].replace(":", ""))

        # Column numbers line
        input()

        # Read anfield
        anfield = []
        for _ in range(rows):
            line = input()
            # Skip row number (format: "000 ..................")
            if len(line) > 4:
                anfield.append(list(line[4:]))

        # Piece header
        line = input().strip()
        if not line.startswith("Piece"):
            return None, None, None

        parts = line.split()
        piece_rows = int(parts[1])
        piece_cols = int(parts[2].replace(":", ""))

        # Read piece
        piece = []
        for _ in range(piece_rows):
            line = input().strip()
            if line:
                piece.append(list(line))

        return player_num, anfield, piece

    except Exception:
        return None, None, None


def get_piece_cells(piece):
    """Get list of (row, col) coordinates of filled cells in piece"""
    cells = []
    for r in range(len(piece)):
        for c in range(len(piece[0])):
            if piece[r][c] in ['O', '#', '*']:
                cells.append((r, c))
    return cells


def is_valid_placement(anfield, piece, pos_row, pos_col, player_num):
    """Check if piece can be placed at position"""
    rows = len(anfield)
    cols = len(anfield[0]) if rows > 0 else 0
    piece_cells = get_piece_cells(piece)

    if player_num == 1:
        my_chars = ['@', 'a']
        opp_chars = ['$', 's']
    else:
        my_chars = ['$', 's']
        opp_chars = ['@', 'a']

    overlap_count = 0

    for pr, pc in piece_cells:
        ar = pos_row + pr
        ac = pos_col + pc

        # Check bounds
        if ar < 0 or ar >= rows or ac < 0 or ac >= cols:
            return False

        cell = anfield[ar][ac]

        # Check overlap with opponent
        if cell in opp_chars:
            return False

        # Count overlap with own territory
        if cell in my_chars:
            overlap_count += 1

    # Must have exactly one overlap
    return overlap_count == 1


def score_placement(anfield, piece, pos_row, pos_col, player_num):
    """Score a placement position"""
    rows = len(anfield)
    cols = len(anfield[0]) if rows > 0 else 0
    piece_cells = get_piece_cells(piece)

    if player_num == 1:
        opp_chars = ['$', 's']
    else:
        opp_chars = ['@', 'a']

    # Find opponent positions
    opp_positions = []
    for r in range(rows):
        for c in range(cols):
            if anfield[r][c] in opp_chars:
                opp_positions.append((r, c))

    if not opp_positions:
        # No opponent yet, place near center
        center_r = rows // 2
        center_c = cols // 2
        avg_r = pos_row + sum(pr for pr, _ in piece_cells) / len(piece_cells)
        avg_c = pos_col + sum(pc for _, pc in piece_cells) / len(piece_cells)
        dist_to_center = math.sqrt((avg_r - center_r)**2 + (avg_c - center_c)**2)
        return -dist_to_center

    # Calculate metrics
    min_dist_to_opp = float('inf')
    total_dist_to_opp = 0

    for pr, pc in piece_cells:
        ar = pos_row + pr
        ac = pos_col + pc

        for or_, oc in opp_positions:
            dist = math.sqrt((ar - or_)**2 + (ac - oc)**2)
            min_dist_to_opp = min(min_dist_to_opp, dist)
            total_dist_to_opp += dist

    avg_dist_to_opp = total_dist_to_opp / (len(piece_cells) * len(opp_positions))

    # Strategy: Balance between blocking opponent and filling board
    # We want to be close enough to block but not too close
    ideal_distance = 3.0
    distance_score = -abs(min_dist_to_opp - ideal_distance)

    # Prefer placements that cover more area efficiently
    coverage_score = len(piece_cells)

    # Prefer placements closer to center of mass of opponent (to block)
    opp_center_r = sum(r for r, _ in opp_positions) / len(opp_positions)
    opp_center_c = sum(c for _, c in opp_positions) / len(opp_positions)

    piece_center_r = pos_row + sum(pr for pr, _ in piece_cells) / len(piece_cells)
    piece_center_c = pos_col + sum(pc for _, pc in piece_cells) / len(piece_cells)

    dist_to_opp_center = math.sqrt((piece_center_r - opp_center_r)**2 +
                                   (piece_center_c - opp_center_c)**2)

    # Combined score
    score = (distance_score * 2.0 +
             coverage_score * 0.5 -
             dist_to_opp_center * 0.5)

    return score


def find_best_placement(anfield, piece, player_num):
    """Find the best placement for the piece"""
    rows = len(anfield)
    cols = len(anfield[0]) if rows > 0 else 0
    piece_rows = len(piece)
    piece_cols = len(piece[0]) if piece_rows > 0 else 0

    best_score = float('-inf')
    best_pos = (0, 0)
    found_valid = False

    # Try all possible positions
    for r in range(-piece_rows + 1, rows):
        for c in range(-piece_cols + 1, cols):
            if is_valid_placement(anfield, piece, r, c, player_num):
                score = score_placement(anfield, piece, r, c, player_num)
                if score > best_score:
                    best_score = score
                    best_pos = (c, r)  # Output format is X Y (col, row)
                    found_valid = True

    return best_pos if found_valid else (0, 0)


def main():
    """Main game loop"""
    while True:
        player_num, anfield, piece = parse_input()

        if player_num is None or anfield is None or piece is None:
            # No more input, exit gracefully
            break

        # Find best placement
        x, y = find_best_placement(anfield, piece, player_num)

        # Output the position
        print(f"{x} {y}")
        sys.stdout.flush()


if __name__ == "__main__":
    main()
