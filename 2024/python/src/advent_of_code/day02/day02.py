class Report:
    _min_allowed_distance = 1
    _max_allowed_distance = 3

    levels: list[int]
    _is_increasing: bool = False

    def __init__(self, data: str) -> None:
        self.levels = [int(x) for x in data.split()]

        self._is_increasing = self.levels[0] < self.levels[1]

    def is_increasing(self) -> bool:
        return self._is_increasing

    def is_decreasing(self) -> bool:
        return not self.is_increasing()

    def _level_distance(self, lhs: int, rhs: int) -> int:
        return abs(lhs - rhs)

    def _level_distance_is_in_range(self, distance: int) -> bool:
        return (
            distance >= self._min_allowed_distance
            and distance <= self._max_allowed_distance
        )

    def is_safe(self) -> bool:
        is_safe = True
        previous_level = self.levels[0]
        for level in self.levels[1:]:
            # Test level distance
            is_safe = is_safe and self._level_distance_is_in_range(
                self._level_distance(previous_level, level)
            )

            # Test if is always increasing
            is_safe = is_safe and (
                level > previous_level if self.is_increasing() else True
            )

            # Test if is always decreasing
            is_safe = is_safe and (
                level < previous_level if self.is_decreasing() else True
            )

            previous_level = level

        return is_safe


def get_report_list(input: list[str]) -> list[Report]:
    return [Report(report) for report in input]


def get_safe_reports_count(reports: list[Report]) -> int:
    count = 0
    for report in reports:
        count += 1 if report.is_safe() else 0

    return count


def part_one(input: list[str]) -> int:
    report_list = get_report_list(input)

    return get_safe_reports_count(report_list)
