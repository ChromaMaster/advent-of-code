import pathlib

from advent_of_code import day01, day02, day03, day04, day05

DATA_PATH = f"{pathlib.Path(__file__).parent.resolve()}/input"


def get_day_input(day: str) -> list[str]:
    input = []
    with open(f"{DATA_PATH}/input-{day}.txt") as f:
        input = f.readlines()

    return [line.strip() for line in input]


def run_day01() -> None:
    input = get_day_input("day01")

    print(f"Day one[part one] solution: {day01.part_one(input)}")
    print(f"Day one[part two] solution: {day01.part_two(input)}")


def run_day02() -> None:
    input = get_day_input("day02")

    print(f"Day two[part one] solution: {day02.part_one(input)}")
    print(f"Day two[part two] solution: {day02.part_two(input)}")


def run_day03() -> None:
    input = get_day_input("day03")

    input = "".join(input)

    print(f"Day three[part one] solution: {day03.part_one(input)}")
    print(f"Day three[part two] solution: {day03.part_two(input)}")


def run_day04() -> None:
    input = get_day_input("day04")

    print(f"Day four[part one] solution: {day04.part_one(input)}")
    print(f"Day four[part two] solution: {day04.part_two(input)}")


def run_day05() -> None:
    input = get_day_input("day05")

    print(f"Day five[part one] solution: {day05.part_one(input)}")


def main() -> None:
    run_day01()
    run_day02()
    run_day03()
    run_day04()
    run_day05()
