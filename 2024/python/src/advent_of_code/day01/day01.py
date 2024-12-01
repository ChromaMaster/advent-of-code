from typing import Tuple


def get_lists_from_input(input: list[str]) -> Tuple[list[int], list[int]]:
    left_list = []
    right_list = []

    for line in input:
        columns = line.split()

        left_list.append(int(columns[0]))
        right_list.append(int(columns[1]))

    return (left_list, right_list)


def sort_lists(
    left_list: list[int], right_list: list[int]
) -> Tuple[list[int], list[int]]:
    return (sorted(left_list), sorted(right_list))


def get_distances_between_elements(
    left_list: list[int], right_list: list[int]
) -> list[int]:
    return [abs(x - y) for x, y in zip(left_list, right_list)]


def get_total_distance(distances: list[int]) -> int:
    return sum(distances)


def get_number_of_occurrences(input: list[int]) -> dict[int, int]:
    occurrences = {}
    for element in input:
        if element not in occurrences:
            occurrences[element] = 1
            continue

        occurrences[element] += 1

    return occurrences


def get_similarity_score(occurrences: dict[int, int], left_list: list[int]) -> int:
    similarity_score = 0
    for element in left_list:
        similarity_score += element * occurrences.get(element, 0)
    return similarity_score


def part_one(input: list[str]) -> int:
    left_list, right_list = get_lists_from_input(input)

    left_list, right_list = sort_lists(left_list, right_list)

    distances = get_distances_between_elements(left_list, right_list)

    return get_total_distance(distances)


def part_two(input: list[str]) -> int:
    left_list, right_list = get_lists_from_input(input)

    occurrences = get_number_of_occurrences(right_list)

    return get_similarity_score(occurrences, left_list)
