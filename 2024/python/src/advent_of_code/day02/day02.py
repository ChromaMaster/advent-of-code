from typing import Self


class Report:
    _min_allowed_distance = 1
    _max_allowed_distance = 3
    _max_faulty_value_allowed = 1

    levels: list[int]
    _is_increasing: bool = False

    def __init__(self, data: str) -> None:
        self.levels = [int(x) for x in data.split()]

        self._is_increasing = self.levels[0] < self.levels[1]

    @classmethod
    def from_int_slice(cls, data: list[int]) -> Self:
        return cls(" ".join([str(x) for x in data]))

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

    def _level_is_faulty(self, level: int, previous_level: int) -> bool:
        if not self._level_distance_is_in_range(
            self._level_distance(level, previous_level)
        ):
            return True

        if self.is_increasing() and level < previous_level:
            return True

        if self.is_decreasing() and level > previous_level:
            return True

        return False

    def _is_safe(self, faulty_removed: int, max_faulty_allowed: int) -> bool:
        previous_level = self.levels[0]
        for i, level in enumerate(self.levels[1:], start=1):
            if self._level_is_faulty(level, previous_level):
                if faulty_removed < max_faulty_allowed:
                    # Corner case where removing the first element the report is correct
                    report_without_first_element = Report.from_int_slice(
                        self.levels[1:]
                    )

                    report_without_previous_level = Report.from_int_slice(
                        self.levels[: i - 1] + self.levels[i:]
                    )

                    report_without_level = Report.from_int_slice(
                        self.levels[:i] + self.levels[i + 1 :]
                    )

                    return (
                        report_without_first_element._is_safe(
                            faulty_removed + 1, max_faulty_allowed
                        )
                        or report_without_previous_level._is_safe(
                            faulty_removed + 1, max_faulty_allowed
                        )
                        or report_without_level._is_safe(
                            faulty_removed + 1, max_faulty_allowed
                        )
                    )

                return False

            previous_level = level

        return True

    def is_safe(self, max_faulty_allowed: int = 0) -> bool:
        return self._is_safe(0, max_faulty_allowed)


def get_report_list(input: list[str]) -> list[Report]:
    return [Report(report) for report in input]


def get_safe_reports_count(reports: list[Report], max_faulty_allowed: int = 0) -> int:
    count = 0

    for report in reports:
        if report.is_safe(max_faulty_allowed):
            count += 1

    return count


def part_one(input: list[str]) -> int:
    report_list = get_report_list(input)

    return get_safe_reports_count(report_list)


def part_two(input: list[str]) -> int:
    report_list = get_report_list(input)

    return get_safe_reports_count(report_list, max_faulty_allowed=1)
