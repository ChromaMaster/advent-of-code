import re


def get_mul_occurrences(input: str) -> list[str]:
    pattern = r"(mul\(\d{1,3},\d{1,3}\))"
    result = re.findall(pattern, input)

    return result


def get_mul_coefficients(mul_sentences: list[str]) -> list[tuple[int, int]]:
    pattern = r"(\d{1,3})"

    coefficients = []
    for mul_sentence in mul_sentences:
        c = re.findall(pattern, mul_sentence)
        coefficients.append((int(c[0]), int(c[1])))

    return coefficients


def operate_mul_coefficients(mul_coefficients: list[tuple[int, int]]) -> int:
    results = [x * y for x, y in mul_coefficients]
    return sum(results)


def part_one(input: str) -> int:
    mul_occurrences = get_mul_occurrences(input)

    mul_coefficients = get_mul_coefficients(mul_occurrences)

    return operate_mul_coefficients(mul_coefficients)
