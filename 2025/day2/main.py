from src.lib import *


def main():
    with open("input.txt") as f:
        input = f.readline()
        ranges = parse_ranges(input)
        print(f"Part 1: {part1(ranges)}")
        print(f"Part 2: {part2(ranges)}")
    print("Hello from day2!")


if __name__ == "__main__":
    main()
