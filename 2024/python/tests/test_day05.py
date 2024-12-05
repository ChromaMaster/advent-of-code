from advent_of_code.day05.day05 import (
    Rule,
    get_rules,
    Update,
    get_updates,
    get_rules_and_updates,
    get_correctly_ordered_updates,
    sum_middle_pages,
)

from advent_of_code.day05 import part_one

problem_input = [
    "47|53",
    "97|13",
    "97|61",
    "97|47",
    "75|29",
    "61|13",
    "75|53",
    "29|13",
    "97|29",
    "53|29",
    "61|53",
    "97|53",
    "61|29",
    "47|13",
    "75|47",
    "97|75",
    "47|61",
    "75|61",
    "47|29",
    "75|13",
    "53|13",
    "",
    "75,47,61,53,29",
    "97,61,53,29,13",
    "75,29,13",
    "75,97,47,61,53",
    "61,13,29",
    "97,13,75,29,47",
]


class TestDay05PartOne:
    def test_it_can_create_a_rule(self) -> None:
        str_rule = "34|11"
        rule = Rule.from_str(str_rule)

        expected_rule = Rule(before=34, after=11)

        assert rule == expected_rule

    def test_it_can_create_a_set_of_rules(self) -> None:
        input = [
            "1|2",
            "3|4",
            "6|5",
        ]

        rules = get_rules(input)

        expected_rules = [
            Rule(before=1, after=2),
            Rule(before=3, after=4),
            Rule(before=6, after=5),
        ]

        assert rules == expected_rules

    def test_it_can_create_an_update(self) -> None:
        str_update = "1,2,3,4"

        update = Update.from_str(str_update)

        expected_update = Update(pages=[1, 2, 3, 4])

        assert update == expected_update

    def test_it_can_create_a_set_of_updates(self) -> None:
        input = [
            "1,2,3",
            "2,3,1",
            "3,5,4",
        ]

        updates = get_updates(input)

        expected_updates = [
            Update(pages=[1, 2, 3]),
            Update(pages=[2, 3, 1]),
            Update(pages=[3, 5, 4]),
        ]

        assert updates == expected_updates

    def test_it_can_create_a_set_of_rules_and_a_set_of_updates_from_a_single_input(
        self,
    ) -> None:
        input = [
            "1|2",
            "3|4",
            "6|5",
            "",
            "1,2,3",
            "2,3,1",
            "3,5,4",
        ]

        (rules, updates) = get_rules_and_updates(input)

        expected_rules = [
            Rule(before=1, after=2),
            Rule(before=3, after=4),
            Rule(before=6, after=5),
        ]

        expected_updates = [
            Update(pages=[1, 2, 3]),
            Update(pages=[2, 3, 1]),
            Update(pages=[3, 5, 4]),
        ]

        assert rules == expected_rules
        assert updates == expected_updates

    def test_an_update_can_evaluate_a_rule_and_ignore_it_if_doesnt_affects_it(
        self,
    ) -> None:
        update = Update(pages=[1, 2, 3])
        rule = Rule(before=5, after=3)

        assert update.evaluate(rule)

    def test_an_update_can_evaluate_a_rule_and_tell_if_its_correctly_ordered(
        self,
    ) -> None:
        update = Update(pages=[1, 2, 3])
        rule = Rule(before=1, after=3)

        assert update.evaluate(rule)

    def test_an_update_can_evaluate_a_rule_and_tell_if_its_NOT_correctly_ordered(
        self,
    ) -> None:
        update = Update(pages=[1, 2, 3])
        rule = Rule(before=2, after=1)

        assert not update.evaluate(rule)

    def test_an_update_can_evaluate_a_set_of_rules_and_tell_if_its_correctly_ordered(
        self,
    ) -> None:
        update = Update(pages=[1, 2, 3])

        rule_set = [
            Rule(before=1, after=3),
            Rule(before=2, after=3),
            Rule(before=1, after=2),
        ]

        assert update.evaluate_set(rule_set)

    def test_an_update_can_return_its_middle_page(self) -> None:
        update = Update(pages=[12, 32, 54])

        expected_middle_page = 32

        assert update.middle_page == expected_middle_page

    def test_an_update_can_evaluate_a_set_of_rules_and_tell_if_its_NOT_correctly_ordered(
        self,
    ) -> None:
        update = Update(pages=[1, 2, 3])

        rule_set = [
            Rule(before=1, after=3),
            Rule(before=1, after=3),
            Rule(before=3, after=2),
        ]

        assert not update.evaluate_set(rule_set)

    def test_it_can_identify_all_the_correctly_ordered_updates_based_on_some_rules(
        self,
    ) -> None:
        rules, updates = get_rules_and_updates(problem_input)

        correctly_ordered_updates = get_correctly_ordered_updates(updates, rules)

        expected_correctly_ordered_updates = [
            Update(pages=[75, 47, 61, 53, 29]),
            Update(pages=[97, 61, 53, 29, 13]),
            Update(pages=[75, 29, 13]),
        ]

        assert correctly_ordered_updates == expected_correctly_ordered_updates

    def test_it_can_sum_the_middle_pages_of_a_set_of_updates(self) -> None:
        updates = [
            Update(pages=[75, 47, 61, 53, 29]),
            Update(pages=[97, 61, 53, 29, 13]),
            Update(pages=[75, 29, 13]),
        ]

        sum = sum_middle_pages(updates)

        expected_sum = 143

        assert sum == expected_sum

    def test_it_can_solve_the_given_problem(self) -> None:
        result = part_one(problem_input)

        expected_result = 143

        assert result == expected_result
