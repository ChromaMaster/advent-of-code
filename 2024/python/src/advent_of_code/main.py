import pathlib

from advent_of_code import day01

DATA_PATH = f"{pathlib.Path(__file__).parent.resolve()}/input"


def main() -> None:
    print("Hello from advent-of-code!")

    input = []
    with open(f"{DATA_PATH}/input-day01.txt") as f:
        input = f.readlines()

    print(f"Day one[part one] solution {day01.part_one(input)}")
    print(f"Day one[part two] solution {day01.part_two(input)}")
