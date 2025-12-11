def parse(file) -> list[str]:
    with open(file) as f:
        input = f.read().splitlines()
    return input


def get_max_joltage_two(batteries: str) -> int:
    max_joltage = 0
    for i in range(len(batteries)):
        for j in range(i + 1, len(batteries)):
            joltage = int(f"{batteries[i]}{batteries[j]}")
            max_joltage = max(joltage, max_joltage)
    return max_joltage


def part1(banks: list[str]) -> int:
    joltages = []
    for batteries in banks:
        joltage = get_max_joltage_two(batteries)
        joltages.append(joltage)
    return sum(joltages)


def get_max_joltage_twelve(batteries: str) -> int:
    cache: dict[tuple[str, int], int] = {}
    return get_max_joltage_x(batteries, 12, cache)


def get_max_joltage_x(
    batteries: str, num_to_enable: int, cache: dict[tuple[str, int], int]
) -> int:
    # dynamic programming
    # cache repeated calls
    if (batteries, num_to_enable) in cache:
        return cache[(batteries, num_to_enable)]
    # base case:
    # - all of the remaining batteries should be turned on
    # - only one battery can be turned on, pick the largest one
    elif num_to_enable == len(batteries):
        return int(batteries)
    elif num_to_enable == 1:
        return max(int(c) for c in batteries)

    # recursive case:
    # first battery is on
    joltage_included = int(
        f"{batteries[0]}{get_max_joltage_x(batteries[1:], num_to_enable - 1, cache)}"
    )

    # first battery is off
    joltage_excluded = int(f"{get_max_joltage_x(batteries[1:], num_to_enable, cache)}")
    max_joltage = max(joltage_included, joltage_excluded)
    cache[(batteries, num_to_enable)] = max_joltage
    return max_joltage


def part2(banks: list[str]) -> int:
    joltages = []
    for batteries in banks:
        joltage = get_max_joltage_twelve(batteries)
        joltages.append(joltage)
        print(batteries, joltage)
    return sum(joltages)
