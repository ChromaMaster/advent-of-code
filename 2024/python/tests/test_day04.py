from advent_of_code.day04.day04 import get_matrix, get_word_occurrences
from advent_of_code.day04 import part_one

problem_input = [
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
        result = part_one(problem_input)

        expected_result = 18

        assert result == expected_result
