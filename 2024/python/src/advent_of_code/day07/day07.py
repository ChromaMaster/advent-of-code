from typing import Self, Any
# from copy import copy


class OperationTrieNode:
    value: int
    operand: int
    next: dict[str, Self] | None

    def __init__(self, value: int, operand: int, next: dict[str, Self] | None) -> None:
        self.value = value
        self.operand = operand
        self.next = next

    @classmethod
    def build(cls, operands: list[int], operators: list[str]) -> Self | None:
        root = cls(operands[0], operands[0], {})

        root.populate_children(1, operands, operators)

        return root

    def _calculate_next_node_value(self, acc: int, operand: int, operator: str) -> int:
        match operator:
            case "+":
                return acc + operand
            case "*":
                return acc * operand
            case _:
                return 0

    def populate_children(
        self, depth: int, operands: list[int], operators: list[str]
    ) -> None:
        if depth >= len(operands):
            return

        operand = operands[depth]
        for operator in operators:
            value = self._calculate_next_node_value(self.value, operand, operator)

            next = OperationTrieNode(value, operand, None)
            next.populate_children(depth + 1, operands, operators)
            if self.next is None:
                self.next = {}

            self.next[operator] = next

    def has(self, value: int) -> bool:
        if self.next is None:
            if self.value == value:
                return True
            else:
                return False

        result: bool = False

        for n in self.next.values():
            result = result or n.has(value)

        return result

    # def path(self, value: int, path: list[str]) -> list[str]:
    #     _local_path = copy(path)
    #     _local_path.append(str(self.operand))
    #
    #     if self.next is None:
    #         if self.value == value:
    #             return _local_path
    #         else:
    #             return []
    #
    #     for k, v in self.next.items():
    #         _branch_path = copy(_local_path)
    #         _branch_path.append(k)
    #         _local_path += v.path(value, _branch_path)
    #
    #     return _local_path

    def as_dict(self) -> dict[str, Any]:
        node_dict = {"value": self.value, "next": {}}

        if self.next is None:
            node_dict["next"] = None
        else:
            for key, value in self.next.items():
                node_dict["next"][key] = value.as_dict()

        return node_dict


class OperationTrie:
    operands: list[int]
    operators: list[str]

    root: OperationTrieNode | None

    def __init__(self, operands: list[int], operators: list[str]) -> None:
        self.operands = operands
        self.operators = operators

        self.root = OperationTrieNode.build(operands, operators)

    def contains_result(self, result: int) -> bool:
        if not self.root:
            return False

        return self.root.has(result)

    # def get_ecuation_for_result(self, result: int) -> str:
    #     if not self.root:
    #         return ""
    #
    #     path = self.root.path(result, [])
    #     print(path)
    #
    #     return str(path)

    def as_dict(self) -> dict[str, Any]:
        if not self.root:
            return {}

        return self.root.as_dict()


class Equation:
    result: int
    operands: list[int]

    def __init__(self, result: int, operands: list[int]) -> None:
        self.result = result
        self.operands = operands

    @classmethod
    def from_str(cls, equation: str) -> Self:
        (result_str, operands_str) = equation.split(":")
        return cls(
            result=int(result_str), operands=[int(o) for o in operands_str.split()]
        )

    def get_operation_trie(self, operators: list[str]) -> OperationTrie:
        return OperationTrie(self.operands, operators)

    def __str__(self) -> str:
        return self.__repr__()

    def __repr__(self) -> str:
        return f"Result: {self.result}, Operands: {self.operands}"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Equation):
            return False

        return self.result == other.result and self.operands == other.operands


def part_one(input: list[str]) -> int:
    equations: list[Equation] = []
    for line in input:
        equations.append(Equation.from_str(line))

    sum = 0
    for equation in equations:
        equation_trie = equation.get_operation_trie(["+", "*"])

        if equation_trie.contains_result(equation.result):
            sum += equation.result

    return sum


# def part_two(input: list[str]) -> int:
#     pass
