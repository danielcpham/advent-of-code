import heapq
import logging
from typing import NamedTuple


class Input(NamedTuple):
    ranges: list[tuple[int, int]]
    ingredients: list[int]


def parse(filename: str) -> Input:
    with open(filename) as f:
        input = f.read().rstrip()
    parts = input.split("\n\n")
    ranges = []
    for r in parts[0].split("\n"):
        nums = r.split("-")
        ranges.append((int(nums[0]), int(nums[1])))
    ingredients = [int(i) for i in parts[1].split("\n")]
    return Input(ranges, ingredients)


def part1(input: Input) -> int:
    num_fresh = 0
    for ingredient in input.ingredients:
        num_fresh += int(any(r[0] <= ingredient <= r[1] for r in input.ranges))
    return num_fresh


def part2(input: Input) -> int:
    ranges = sorted(input.ranges)
    combined_ranges = [ranges[0]]
    logging.debug(f"ranges: {combined_ranges}")
    for r in ranges[1:]:
        last_range = combined_ranges[0]
        logging.debug(f"{last_range=}, new_range={r}")
        # if the new range starts inside the previous range,
        # combine two ranges
        if r[0] <= last_range[1]:
            new_range = (last_range[0], max(r[1], last_range[1]))
            logging.debug(f"combined_range={new_range}")
            heapq.heappop_max(combined_ranges)
            heapq.heappush_max(combined_ranges, new_range)
        else:
            heapq.heappush_max(combined_ranges, r)
        logging.debug(f"ranges: {combined_ranges}")

    total_fresh_ids = 0
    for r in combined_ranges:
        total_fresh_ids += r[1] - r[0] + 1
    return total_fresh_ids
