from advent_of_code.day06.day06 import (
    Point,
    Vector,
    Direction,
    Guard,
    get_guard,
    get_obstacles,
    is_collision,
    guard_is_outside_the_map_area,
    get_map,
)

from advent_of_code.day06 import part_one

problem_input = [
    "....#.....",
    ".........#",
    "..........",
    "..#.......",
    ".......#..",
    "..........",
    ".#..^.....",
    "........#.",
    "#.........",
    "......#...",
]


class TestDay05PartOne:
    def test_it_can_create_a_guard_with_a_starting_position_and_direction(self) -> None:
        guard = Guard(Point(x=1, y=1), Direction.UP)

        expected_position = Point(x=1, y=1)
        expected_direction = Vector(x=0, y=-1)

        assert guard.position == expected_position
        assert guard.direction.value == expected_direction

    def test_the_guard_can_move_forward(self) -> None:
        guard = Guard(Point(x=1, y=1), Direction.UP)

        guard.move()

        expected_position = Point(x=1, y=0)

        assert guard.position == expected_position

    def test_the_next_position_of_the_guard_can_be_retrieved(self) -> None:
        guard = Guard(Point(x=1, y=1), Direction.DOWN)

        expected_position = Point(x=1, y=2)

        assert guard.next_position() == expected_position

    def test_the_guard_can_turn_right(self) -> None:
        guard = Guard(Point(x=1, y=1), Direction.DOWN)

        guard.turn_right()

        expected_direction = Direction.LEFT

        assert guard.direction == expected_direction

    def test_the_guard_keeps_track_of_visited_positions(self) -> None:
        guard = Guard(Point(x=1, y=1), Direction.DOWN)

        guard.move()  # Moves down
        guard.turn_right()  # Turns right, points left
        guard.move()  # Moves left
        guard.move()  # Moves left
        guard.turn_right()  # Turns right, points up
        guard.turn_right()  # Turns right, points right
        guard.move()  # Moves right (this should not cound twice)

        expected_visited_positions = {
            Point(x=1, y=1),
            Point(x=1, y=2),
            Point(x=0, y=2),
            Point(x=-1, y=2),
        }

        assert guard.visited_positions == expected_visited_positions

    def test_it_can_get_the_guard_from_a_map(self) -> None:
        input = [
            [".", ".", ".", "."],
            [".", ".", ".", "."],
            [".", ".", ".", "^"],
            [".", ".", ".", "."],
        ]

        guard = get_guard(input, "^")

        print(guard)

        expected_guard = Guard(Point(x=3, y=2), Direction.UP)

        assert guard == expected_guard

    def test_it_can_get_all_obstacles_from_a_map(self) -> None:
        input = [
            [".", "#", "#", "."],
            [".", "#", ".", "."],
            [".", ".", "#", "."],
            ["#", ".", ".", "#"],
        ]

        obstacles = get_obstacles(input, "#")

        expected_obstacles = {
            Point(x=1, y=0): True,
            Point(x=2, y=0): True,
            Point(x=1, y=1): True,
            Point(x=2, y=2): True,
            Point(x=0, y=3): True,
            Point(x=3, y=3): True,
        }

        assert obstacles == expected_obstacles

    def test_it_can_identify_a_collision(self) -> None:
        guard_next_position = Point(x=2, y=3)
        obstacle_position = Point(x=2, y=3)

        assert is_collision(guard_next_position, obstacle_position)

    def test_is_possible_to_know_if_the_guard_is_outside_the_map_area(self) -> None:
        guard = Guard(Point(x=1, y=3), Direction.UP)

        boundaries = (3, 3)

        assert guard_is_outside_the_map_area(guard, boundaries)

    def test_is_possible_to_know_if_the_guard_is_inside_the_map_area(self) -> None:
        guard = Guard(Point(x=1, y=2), Direction.UP)

        boundaries = (3, 3)

        assert not guard_is_outside_the_map_area(guard, boundaries)

    def test_it_can_get_a_map_from_a_list_of_str_lines(self) -> None:
        input = [
            ".##.",
            ".#..",
            "..#.",
            "#^.#",
        ]

        map = get_map(input)

        expected_map = [
            [".", "#", "#", "."],
            [".", "#", ".", "."],
            [".", ".", "#", "."],
            ["#", "^", ".", "#"],
        ]

        assert map == expected_map

    def test_it_can_solve_the_given_problem(self) -> None:
        result = part_one(problem_input)

        expected_result = 41

        assert result == expected_result
