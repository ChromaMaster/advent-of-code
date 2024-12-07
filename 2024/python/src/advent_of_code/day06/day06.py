from dataclasses import dataclass
from enum import Enum
from typing import Self

import copy


@dataclass
class Vector:
    x: int
    y: int


class Direction(Enum):
    UP = Vector(x=0, y=-1)
    RIGHT = Vector(x=1, y=0)
    DOWN = Vector(x=0, y=1)
    LEFT = Vector(x=-1, y=0)

    @classmethod
    def next_clockwise_direction(cls, direction: Self) -> Self:
        match direction:
            case cls.UP:
                return cls(cls.RIGHT)
            case cls.RIGHT:
                return cls(cls.DOWN)
            case cls.DOWN:
                return cls(cls.LEFT)
            case cls.LEFT:
                return cls(cls.UP)


@dataclass
class Point:
    x: int
    y: int

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    def move(self, direction: Direction) -> None:
        self.x, self.y = self.next_position(direction)

    def next_position(self, direction: Direction) -> tuple[int, int]:
        return self.x + direction.value.x, self.y + direction.value.y


class Guard:
    position: Point
    direction: Direction
    visited_positions: set[Point]

    def __init__(self, position: Point, direction: Direction) -> None:
        self.position = position
        self.direction = direction

        # Add starting position
        self.visited_positions = set()
        # self.visited_positions.add(copy.deepcopy(self.position))

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Guard):
            return False

        return (
            (self.position == other.position)
            and (self.direction == self.direction)
            and (self.visited_positions == other.visited_positions)
        )

    def __str__(self) -> str:
        return self.__repr__()

    def __repr__(self) -> str:
        return f"Position: {self.position}, Direction: {self.direction}"
        # return f"Position: {self.position}, Direction: {self.direction}, Visited positions: {self.visited_positions}"

    def next_position(self) -> Point:
        return Point(*self.position.next_position(self.direction))

    def move(self) -> None:
        self.visited_positions.add(copy.deepcopy(self.position))
        self.position.move(self.direction)

    def turn_right(self) -> None:
        self.direction = Direction.next_clockwise_direction(self.direction)


def get_guard(map: list[list[str]], guard_symbol: str) -> Guard | None:
    for row in range(len(map)):
        for col in range(len(map[row])):
            if map[row][col] == guard_symbol:
                return Guard(Point(x=col, y=row), Direction.UP)

    return None


def get_obstacles(map: list[list[str]], obstacle_symbol: str) -> dict[Point, bool]:
    obstacles = {}
    for row in range(len(map)):
        for col in range(len(map[row])):
            if map[row][col] == obstacle_symbol:
                obstacles[Point(x=col, y=row)] = True

    return obstacles


def is_collision(guard_position: Point, obstacle_position: Point) -> bool:
    return guard_position == obstacle_position


def guard_is_outside_the_map_area(guard: Guard, boundaries: tuple[int, int]) -> bool:
    return (guard.position.y < 0 or guard.position.y >= boundaries[0]) or (
        guard.position.x < 0 or guard.position.x >= boundaries[1]
    )


def get_map(input: list[str]) -> list[list[str]]:
    map = []

    for line in input:
        map.append([i for i in line])

    return map


def part_one(input: list[str]) -> int:
    map = get_map(input)
    map_boundaries = (len(map), len(map[0]))

    guard = get_guard(map, "^")
    if not guard:
        return 0

    obstacles = get_obstacles(map, "#")

    while not guard_is_outside_the_map_area(guard, map_boundaries):
        if guard.next_position() in obstacles:
            guard.turn_right()

        guard.move()

    return len(guard.visited_positions)


# def part_two(input: list[str]) -> int:
