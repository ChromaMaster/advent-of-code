from advent_of_code.day03.day03 import (
    get_mul_occurrences,
    get_mul_coefficients,
    operate_mul_coefficients,
)


from advent_of_code.day03 import part_one


problem_input = (
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
)


class TestDay03PartOne:
    def test_it_can_split_the_input_isolating_all_the_mul_ocurrences(self) -> None:
        mul_sentences = get_mul_occurrences(problem_input)

        expected_mul_sentences = [
            "mul(2,4)",
            "mul(5,5)",
            "mul(11,8)",
            "mul(8,5)",
        ]

        assert mul_sentences == expected_mul_sentences

    def test_it_can_obtain_the_mul_coefficients(self) -> None:
        mul_sentences = [
            "mul(2,4)",
            "mul(5,5)",
            "mul(11,8)",
            "mul(8,5)",
        ]

        mul_coefficients = get_mul_coefficients(mul_sentences)

        expected_mul_coefficients = [(2, 4), (5, 5), (11, 8), (8, 5)]

        assert mul_coefficients == expected_mul_coefficients

    def test_it_can_multiply_and_add_the_mul_coefficients_results(self) -> None:
        mul_coefficients = [(2, 4), (5, 5), (11, 8), (8, 5)]

        result = operate_mul_coefficients(mul_coefficients)

        expected_result = 161

        assert result == expected_result

    def test_it_can_solve_the_given_problem(self) -> None:
        result = part_one(problem_input)

        expected_result = 161

        assert result == expected_result
