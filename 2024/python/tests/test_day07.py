from advent_of_code.day07.day07 import Equation, OperationTrie
from advent_of_code.day07 import part_one, part_two

problem_input = [
    "190: 10 19",
    "3267: 81 40 27",
    "83: 17 5",
    "156: 15 6",
    "7290: 6 8 6 15",
    "161011: 16 10 13",
    "192: 17 8 14",
    "21037: 9 7 18 13",
    "292: 11 6 16 20",
]


class TestDay07PartOne:
    def test_it_can_get_an_equation_with_the_result_and_operands(self) -> None:
        equation = Equation.from_str("6: 1 2 3")

        expected_equation = Equation(result=6, operands=[1, 2, 3])

        assert equation == expected_equation

    def test_it_can_represent_an_operation_trie_with_a_single_operator(self) -> None:
        trie = OperationTrie(operands=[1, 2, 3], operators=["+"])

        expected_trie_repr = {
            "value": 1,
            "next": {"+": {"value": 3, "next": {"+": {"value": 6, "next": None}}}},
        }

        assert trie.as_dict() == expected_trie_repr

    def test_it_can_represent_an_operation_trie_with_two_operators(self) -> None:
        trie = OperationTrie(operands=[1, 2, 3], operators=["+", "*"])

        expected_trie_repr = {
            "value": 1,
            "next": {
                "+": {
                    "value": 3,
                    "next": {
                        "+": {
                            "value": 6,
                            "next": None,
                        },
                        "*": {
                            "value": 9,
                            "next": None,
                        },
                    },
                },
                "*": {
                    "value": 2,
                    "next": {
                        "+": {
                            "value": 5,
                            "next": None,
                        },
                        "*": {
                            "value": 6,
                            "next": None,
                        },
                    },
                },
            },
        }

        assert trie.as_dict() == expected_trie_repr

    def test_it_can_create_a_operation_trie_from_an_equation(self) -> None:
        equation = Equation(6, [1, 2, 3])

        trie = equation.get_operation_trie(["+", "*"])
        expected_trie_repr = {
            "value": 1,
            "next": {
                "+": {
                    "value": 3,
                    "next": {
                        "+": {
                            "value": 6,
                            "next": None,
                        },
                        "*": {
                            "value": 9,
                            "next": None,
                        },
                    },
                },
                "*": {
                    "value": 2,
                    "next": {
                        "+": {
                            "value": 5,
                            "next": None,
                        },
                        "*": {
                            "value": 6,
                            "next": None,
                        },
                    },
                },
            },
        }

        assert trie.as_dict() == expected_trie_repr

    def test_it_can_find_a_result_in_a_trie(self) -> None:
        trie = OperationTrie(operands=[1, 2, 3], operators=["+", "*"])

        assert trie.contains_result(5)  # 1 * 2 + 3

    # def test_it_can_obtain_the_operation_for_an_existing_result(self) -> None:
    #     trie = OperationTrie(operands=[1, 2, 3], operators=["+", "*"])
    #
    #     assert trie.get_ecuation_for_result(5) == "1 * 2 + 3"

    def test_it_can_solve_the_given_problem(self) -> None:
        result = part_one(problem_input)

        expected_result = 3749

        assert result == expected_result


class TestDay07PartTwo:
    def test_it_can_create_an_operation_trie_with_the_concat_operator(self) -> None:
        trie = OperationTrie(operands=[1, 2, 3], operators=["+", "||"])

        expected_trie_repr = {
            "value": 1,
            "next": {
                "+": {
                    "value": 3,
                    "next": {
                        "+": {
                            "value": 6,
                            "next": None,
                        },
                        "||": {
                            "value": 33,
                            "next": None,
                        },
                    },
                },
                "||": {
                    "value": 12,
                    "next": {
                        "+": {
                            "value": 15,
                            "next": None,
                        },
                        "||": {
                            "value": 123,
                            "next": None,
                        },
                    },
                },
            },
        }

        assert trie.as_dict() == expected_trie_repr

    def test_it_can_solve_the_given_problem(self) -> None:
        result = part_two(problem_input)

        expected_result = 11387

        assert result == expected_result
