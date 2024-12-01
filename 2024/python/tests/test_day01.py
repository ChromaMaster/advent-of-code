from advent_of_code.day01.day01 import (
    get_lists_from_input,
    get_distances_between_elements,
    get_total_distance,
    sort_lists,
)

from advent_of_code.day01 import part_one

part_one_input = [
    "3   4",
    "4   3",
    "2   5",
    "1   3",
    "3   9",
    "3   3",
]


class TestDay01PartOne:
    def test_it_can_split_the_input_into_two_lists(self) -> None:
        left_list, right_list = get_lists_from_input(part_one_input)

        expected_left_list = [3, 4, 2, 1, 3, 3]
        expected_right_list = [4, 3, 5, 3, 9, 3]

        assert left_list == expected_left_list
        assert right_list == expected_right_list

    def test_it_can_sort_the_lists(self) -> None:
        left_list = [3, 4, 2, 1, 3, 3]
        right_list = [4, 3, 5, 3, 9, 3]

        left_list, right_list = sort_lists(left_list, right_list)

        expected_left_list = [1, 2, 3, 3, 3, 4]
        expected_right_list = [3, 3, 3, 4, 5, 9]

        assert left_list == expected_left_list
        assert right_list == expected_right_list

    def test_it_can_get_the_distance_of_elements_in_a_sorted_list(self) -> None:
        sorted_left_list = [1, 2, 3, 3, 3, 4]
        sorted_right_list = [3, 3, 3, 4, 5, 9]

        distances = get_distances_between_elements(sorted_left_list, sorted_right_list)

        expected_distances = [2, 1, 0, 1, 2, 5]

        assert distances == expected_distances

    def test_it_can_get_the_sum_of_distances(self) -> None:
        distances = [2, 1, 0, 1, 2, 5]

        total_distance = get_total_distance(distances)

        expexted_total_distance = 11

        assert total_distance == expexted_total_distance

    def test_it_can_solve_the_given_problem(self) -> None:
        total_distance = part_one(part_one_input)

        expexted_total_distance = 11

        assert total_distance == expexted_total_distance
