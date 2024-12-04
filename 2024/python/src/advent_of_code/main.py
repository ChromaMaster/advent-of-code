import pathlib

from advent_of_code import day01, day02, day03, day04

DATA_PATH = f"{pathlib.Path(__file__).parent.resolve()}/input"


def run_day01() -> None:
    input = []
    with open(f"{DATA_PATH}/input-day01.txt") as f:
        input = f.readlines()

    print(f"Day one[part one] solution: {day01.part_one(input)}")
    print(f"Day one[part two] solution: {day01.part_two(input)}")


def run_day02() -> None:
    input = []
    with open(f"{DATA_PATH}/input-day02.txt") as f:
        input = f.readlines()

    print(f"Day two[part one] solution: {day02.part_one(input)}")
    print(f"Day two[part two] solution: {day02.part_two(input)}")


def run_day03() -> None:
    input = []
    with open(f"{DATA_PATH}/input-day03.txt") as f:
        input = f.readlines()

    input = "".join(input)

    print(f"Day two[part one] solution: {day03.part_one(input)}")
    print(f"Day two[part two] solution: {day03.part_two(input)}")


def run_day04() -> None:
    input = []
    with open(f"{DATA_PATH}/input-day04.txt") as f:
        input = f.readlines()

    print(f"Day four[part one] solution: {day04.part_one(input)}")


def main() -> None:
    run_day04()
