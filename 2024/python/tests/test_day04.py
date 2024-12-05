from advent_of_code.day04.day04 import (
    get_matrix,
    get_word_occurrences,
    get_x_max_occurrences,
)
from advent_of_code.day04 import part_one, part_two

problem_input_part_one = [
    "MMMSXXMASM",
    "MSAMXMSMSA",
    "AMXSXMAAMM",
    "MSAMASMSMX",
    "XMASAMXAMM",
    "XXAMMXXAMA",
    "SMSMSASXSS",
    "SAXAMASAAA",
    "MAMMMXMMMM",
    "MXMXAXMASX",
]
problem_input_part_two = [
    ".M.S......",
    "..A..MSMS.",
    ".M.S.MAA..",
    "..A.ASMSM.",
    ".M.S.M....",
    "..........",
    "S.S.S.S.S.",
    ".A.A.A.A..",
    "M.M.M.M.M.",
    "..........",
]


class TestDay04PartOne:
    def test_it_is_able_to_create_a_matrix_with_all_the_info(self) -> None:
        input = ["abc", "def", "ghi"]
        matrix = get_matrix(input)

        expexted_matrix = [
            ["a", "b", "c"],
            ["d", "e", "f"],
            ["g", "h", "i"],
        ]

        assert matrix == expexted_matrix

    def test_it_is_able_to_find_a_word_horizontally(self) -> None:
        matrix = [["a", "b", "c"]]

        occurrences = get_word_occurrences(matrix, "abc")

        expected_occurrences = 1

        assert occurrences == expected_occurrences

    def test_it_is_able_to_find_a_word_vertically(self) -> None:
        matrix = [["a"], ["b"], ["c"]]

        occurrences = get_word_occurrences(matrix, "abc")

        expected_occurrences = 1

        assert occurrences == expected_occurrences

    def test_it_is_able_to_find_a_word_diagonally(self) -> None:
        matrix = [
            ["a", "x", "x"],
            ["x", "b", "x"],
            ["x", "x", "c"],
        ]

        occurrences = get_word_occurrences(matrix, "abc")

        expected_occurrences = 1

        assert occurrences == expected_occurrences

    def test_it_is_able_to_find_a_word_inverted_horizontally(self) -> None:
        matrix = [["c", "b", "a"]]

        occurrences = get_word_occurrences(matrix, "abc")

        expected_occurrences = 1

        assert occurrences == expected_occurrences

    def test_it_is_able_to_find_a_word_inverted_vertically(self) -> None:
        matrix = [["c"], ["b"], ["a"]]

        occurrences = get_word_occurrences(matrix, "abc")

        expected_occurrences = 1

        assert occurrences == expected_occurrences

    def test_it_is_able_to_find_a_word_inverted_diagonally(self) -> None:
        matrix = [
            ["c", "x", "x"],
            ["x", "b", "x"],
            ["x", "x", "a"],
        ]

        occurrences = get_word_occurrences(matrix, "abc")

        expected_occurrences = 1

        assert occurrences == expected_occurrences

    def test_it_is_able_to_find_all_word_occurrences(self) -> None:
        matrix = [
            [".", ".", ".", ".", "X", "X", "M", "A", "S", "."],
            [".", "S", "A", "M", "X", "M", "S", ".", ".", "."],
            [".", ".", ".", "S", ".", ".", "A", ".", ".", "."],
            [".", ".", "A", ".", "A", ".", "M", "S", ".", "X"],
            ["X", "M", "A", "S", "A", "M", "X", ".", "M", "M"],
            ["X", ".", ".", ".", ".", ".", "X", "A", ".", "A"],
            ["S", ".", "S", ".", "S", ".", "S", ".", "S", "S"],
            [".", "A", ".", "A", ".", "A", ".", "A", ".", "A"],
            [".", ".", "M", ".", "M", ".", "M", ".", "M", "M"],
            [".", "X", ".", "X", ".", "X", "M", "A", "S", "X"],
        ]

        occurrences = get_word_occurrences(matrix, "XMAS")

        expected_occurrences = 18

        assert occurrences == expected_occurrences

    def test_it_can_solve_the_given_problem(self) -> None:
        result = part_one(problem_input_part_one)

        expected_result = 18

        assert result == expected_result


class TestDay04PartTwo:
    def test_it_is_able_to_find_all_x_mas_ocurrences(self) -> None:
        matrix = [
            [".", "M", ".", "S", ".", ".", ".", ".", ".", "."],
            [".", ".", "A", ".", ".", "M", "S", "M", "S", "."],
            [".", "M", ".", "S", ".", "M", "A", "A", ".", "."],
            [".", ".", "A", ".", "A", "S", "M", "S", "M", "."],
            [".", "M", ".", "S", ".", "M", ".", ".", ".", "."],
            [".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
            ["S", ".", "S", ".", "S", ".", "S", ".", "S", "."],
            [".", "A", ".", "A", ".", "A", ".", "A", ".", "."],
            ["M", ".", "M", ".", "M", ".", "M", ".", "M", "."],
            [".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        ]

        ocurrences = get_x_max_occurrences(matrix)

        expected_ocurrences = 9

        assert ocurrences == expected_ocurrences

    def test_it_can_solve_the_given_problem(self) -> None:
        result = part_two(problem_input_part_two)

        expected_result = 9

        assert result == expected_result
