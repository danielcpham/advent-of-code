import logging


def parse(file):
    with open(file) as f:
        input = f.readline()
        ranges = parse_ranges(input)
    return ranges


def parse_ranges(input: str) -> list[tuple[int, int]]:
    ranges = input.split(",")
    parsed_ranges = [r.split("-") for r in ranges]
    return [(int(p[0]), int(p[1])) for p in parsed_ranges]


def is_sequence_repeated_twice(s: str) -> bool:
    middle = len(s) // 2
    return s[:middle] == s[middle:]


def part1(ranges: list[tuple[int, int]]) -> int:
    invalid_nums = []
    for r in ranges:
        for num in range(r[0], r[1] + 1):
            if is_sequence_repeated_twice(str(num)):
                invalid_nums.append(num)
    return sum(invalid_nums)


def is_repeated_sequence(s: str) -> bool:
    starting_character = s[0]
    # get indexes of the same character
    matching_indexes = [i for i, c in enumerate(s) if c == s[0] and i != 0]
    for i in matching_indexes:
        # build substrings that could be potential matches
        substring = s[0:i]
        # get num times to repeat
        repeat_times = len(s) // len(substring)
        # does s only consist of the substring repeated?
        repeated_string = substring * repeat_times
        if repeated_string == s:
            return True
    return False


def part2(ranges: list[tuple[int, int]]) -> int:
    invalid_nums = []
    for r in ranges:
        for num in range(r[0], r[1] + 1):
            if is_repeated_sequence(str(num)):
                invalid_nums.append(num)
    logging.debug(invalid_nums)
    return sum(invalid_nums)
