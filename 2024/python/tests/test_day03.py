from advent_of_code.day03.day03 import (
    get_mul_occurrences,
    get_mul_coefficients,
    operate_mul_coefficients,
    get_program_sentences,
    filter_sentences,
)


from advent_of_code.day03 import part_one, part_two


problem_input = (
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
)

problem_input_with_conditionals = (
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
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


class TestDay03PartTwo:
    def test_it_can_split_the_input_isolating_all_the_mul_and_conditional_occurrences(
        self,
    ) -> None:
        sentences = get_program_sentences(problem_input_with_conditionals)

        expected_sentences = [
            "mul(2,4)",
            "don't",
            "mul(5,5)",
            "mul(11,8)",
            "do",
            "mul(8,5)",
        ]

        assert sentences == expected_sentences

    def test_it_can_filter_out_muls_that_are_behind_a_dont_sentences(self) -> None:
        sentences = [
            "mul(2,4)",
            "don't",
            "mul(5,5)",
            "mul(11,8)",
            "do",
            "mul(8,5)",
        ]

        applicable_sentences = filter_sentences(sentences)

        expected_applicable_sentences = [
            "mul(2,4)",
            "mul(8,5)",
        ]

        assert applicable_sentences == expected_applicable_sentences

    def test_it_can_solve_the_given_problem(self) -> None:
        result = part_two(problem_input_with_conditionals)

        expected_result = 48

        assert result == expected_result
