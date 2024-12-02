import pathlib

from advent_of_code import day01, day02

DATA_PATH = f"{pathlib.Path(__file__).parent.resolve()}/input"


def run_day01() -> None:
    input = []
    with open(f"{DATA_PATH}/input-day01.txt") as f:
        input = f.readlines()

    print(f"Day one[part one] solution {day01.part_one(input)}")
    print(f"Day one[part two] solution {day01.part_two(input)}")


def run_day02() -> None:
    input = []
    with open(f"{DATA_PATH}/input-day02.txt") as f:
        input = f.readlines()

    print(f"Day two[part one] solution {day02.part_one(input)}")


def main() -> None:
    run_day02()
