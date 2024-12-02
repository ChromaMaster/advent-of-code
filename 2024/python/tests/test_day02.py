from advent_of_code.day02.day02 import Report, get_report_list, get_safe_reports_count

from advent_of_code.day02 import part_one, part_two

problem_input = [
    "7 6 4 2 1",
    "1 2 7 8 9",
    "9 7 6 2 1",
    "1 3 2 4 5",
    "8 6 4 4 1",
    "1 3 6 7 9",
]


class TestDay02PartOne:
    def test_it_can_identify_if_a_report_is_increasing_or_decreasing(self) -> None:
        increasing_report = Report("1 2")
        decreasing_report = Report("4 3")

        assert increasing_report.is_increasing()
        assert decreasing_report.is_decreasing()

    def test_it_can_identify_if_a_report_is_safe(self) -> None:
        increasing_safe_report = Report("1 2 4 7 9 11 12")
        decreasing_safe_report = Report("12 11 9 7 4 2 1")

        assert increasing_safe_report.is_safe()
        assert decreasing_safe_report.is_safe()

    def test_it_can_identify_if_a_report_is_not_safe(self) -> None:
        increasing_unsafe_report = Report("1 2 4 7 11 12")
        decreasing_unsafe_report = Report("12 11 9 6 2 1")

        assert not increasing_unsafe_report.is_safe()
        assert not decreasing_unsafe_report.is_safe()

    def it_can_get_a_list_of_reports_from_a_reports_string(self) -> None:
        report_list = get_report_list(problem_input)

        expected_report_list = [
            Report("7 6 4 2 1"),
            Report("1 2 7 8 9"),
            Report("9 7 6 2 1"),
            Report("1 3 2 4 5"),
            Report("8 6 4 4 1"),
            Report("1 3 6 7 9"),
        ]

        assert report_list == expected_report_list

    def test_it_can_count_the_safe_reports_within_a_bunch_of_them(self) -> None:
        report_list = [
            Report("7 6 4 2 1"),
            Report("1 2 7 8 9"),
            Report("9 7 6 2 1"),
            Report("1 3 2 4 5"),
            Report("8 6 4 4 1"),
            Report("1 3 6 7 9"),
        ]
        safe_reports_count = get_safe_reports_count(report_list)

        expected_safe_reports_count = 2

        assert safe_reports_count == expected_safe_reports_count

    def test_it_can_solve_the_given_problem(self) -> None:
        safe_report_count = part_one(problem_input)

        expected_safe_report_count = 2

        assert safe_report_count == expected_safe_report_count


class TestDay02PartTwo:
    def test_it_allows_a_number_of_faulty_levels_to_consider_a_report_safe(
        self,
    ) -> None:
        report = Report("4 2 6 3 4 7 8")

        assert report.is_safe(max_faulty_allowed=2)

    def test_it_can_count_the_safe_reports_within_a_bunch_of_them_with_a_faulty_threshold(
        self,
    ) -> None:
        report_list = [
            Report("7 6 4 2 1"),
            Report("1 2 7 8 9"),
            Report("9 7 6 2 1"),
            Report("1 3 2 4 5"),
            Report("8 6 4 4 1"),
            Report("1 3 6 7 9"),
        ]
        safe_reports_count = get_safe_reports_count(report_list, 1)

        expected_safe_reports_count = 4

        assert safe_reports_count == expected_safe_reports_count

    def test_it_can_solve_the_given_problem(self) -> None:
        safe_report_count = part_two(problem_input)

        expected_safe_report_count = 4

        assert safe_report_count == expected_safe_report_count
