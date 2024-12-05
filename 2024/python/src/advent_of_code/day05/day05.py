from dataclasses import dataclass
from typing import Self
import math


@dataclass
class Rule:
    before: int
    after: int

    @classmethod
    def from_str(cls, input: str) -> Self:
        (before, after) = input.split("|")

        return cls(before=int(before), after=int(after))


class Update:
    pages: list[int]

    def __init__(self, pages: list[int]) -> None:
        self.pages = pages

    @classmethod
    def from_str(cls, input: str) -> Self:
        return cls(pages=[int(page) for page in input.split(",")])

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Update):
            return False

        return self.pages == other.pages

    @property
    def middle_page(self) -> int:
        return self.pages[math.floor(len(self.pages) / 2)]

    def evaluate(self, rule: Rule) -> bool:
        if rule.before not in self.pages or rule.after not in self.pages:
            return True

        before_index = self.pages.index(rule.before)
        after_index = self.pages.index(rule.after)

        return before_index < after_index

    def evaluate_set(self, rules: list[Rule]) -> bool:
        for rule in rules:
            if not self.evaluate(rule):
                return False

        return True


def get_rules(lines: list[str]) -> list[Rule]:
    rules = []

    for line in lines:
        rules.append(Rule.from_str(line))

    return rules


def get_updates(lines: list[str]) -> list[Update]:
    updates = []

    for line in lines:
        updates.append(Update.from_str(line))

    return updates


def get_rules_and_updates(lines: list[str]) -> tuple[list[Rule], list[Update]]:
    separation = lines.index("")
    return (get_rules(lines[:separation]), get_updates(lines[separation + 1 :]))


def get_correctly_ordered_updates(
    updates: list[Update], rules: list[Rule]
) -> list[Update]:
    correctly_ordered_updates = []

    for update in updates:
        if update.evaluate_set(rules):
            correctly_ordered_updates.append(update)

    return correctly_ordered_updates


def sum_middle_pages(updates: list[Update]) -> int:
    sum = 0

    for update in updates:
        sum += update.middle_page

    return sum


def part_one(input: list[str]) -> int:
    rules, updates = get_rules_and_updates(input)

    correctly_ordered_updates = get_correctly_ordered_updates(updates, rules)

    return sum_middle_pages(correctly_ordered_updates)
