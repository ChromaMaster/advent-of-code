from dataclasses import dataclass


@dataclass
class Direction:
    col: int
    row: int


def get_matrix(input: list[str]) -> list[list[str]]:
    matrix = []

    for row in input:
        matrix.append(list(row))

    return matrix


def _get_word_occurrences(
    matrix: list[list[str]], word: str, acc: str, dir: Direction, col: int, row: int
) -> int:
    acc += matrix[col][row]
    if acc == word:
        return 1

    if len(acc) >= len(word):
        return 0

    next_col = col + dir.col
    next_row = row + dir.row

    if (next_col < 0 or next_col >= len(matrix)) or (
        next_row < 0 or next_row >= len(matrix[col])
    ):
        return 0

    return _get_word_occurrences(matrix, word, acc, dir, next_col, next_row)


def get_word_occurrences(matrix: list[list[str]], word: str) -> int:
    occurrences = 0
    for col in range(len(matrix)):
        for row in range(len(matrix[col])):
            # Horizontal
            ## Left to Right
            occurrences += _get_word_occurrences(
                matrix, word, "", Direction(col=0, row=1), col, row
            )
            ## Right to Left
            occurrences += _get_word_occurrences(
                matrix, word, "", Direction(col=0, row=-1), col, row
            )

            # Vertical
            ## Top to Bottom
            occurrences += _get_word_occurrences(
                matrix, word, "", Direction(col=1, row=0), col, row
            )
            ## Bottom to Top
            occurrences += _get_word_occurrences(
                matrix, word, "", Direction(col=-1, row=0), col, row
            )

            # Diagonal
            ## Left to Right Top to Bottom
            occurrences += _get_word_occurrences(
                matrix, word, "", Direction(col=1, row=1), col, row
            )
            ## Right to Left Top to Bottom
            occurrences += _get_word_occurrences(
                matrix, word, "", Direction(col=1, row=-1), col, row
            )
            ## Right to Left Bottom to Top
            occurrences += _get_word_occurrences(
                matrix, word, "", Direction(col=-1, row=-1), col, row
            )
            ## Left to Right Bottom to Top
            occurrences += _get_word_occurrences(
                matrix, word, "", Direction(col=-1, row=1), col, row
            )

    return occurrences


def get_x_max_occurrences(matrix) -> int:
    cross = {}
    word = "MAS"
    for col in range(len(matrix)):
        for row in range(len(matrix[col])):
            key = f"{col+1},{row+1}"
            cross[key] = cross.get(key, 0) + _get_word_occurrences(
                matrix, word, "", Direction(col=1, row=1), col, row
            )

            key = f"{col-1},{row-1}"
            cross[key] = cross.get(key, 0) + _get_word_occurrences(
                matrix, word, "", Direction(col=-1, row=-1), col, row
            )

            key = f"{col+1},{row-1}"
            cross[key] = cross.get(key, 0) + _get_word_occurrences(
                matrix, word, "", Direction(col=1, row=-1), col, row
            )

            key = f"{col-1},{row+1}"
            cross[key] = cross.get(key, 0) + _get_word_occurrences(
                matrix, word, "", Direction(col=-1, row=1), col, row
            )

    return len([key for key, value in cross.items() if value == 2])


def part_one(input: list[str]) -> int:
    matrix = get_matrix(input)

    return get_word_occurrences(matrix, "XMAS")


def part_two(input: list[str]) -> int:
    matrix = get_matrix(input)

    return get_x_max_occurrences(matrix)
