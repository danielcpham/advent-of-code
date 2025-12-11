import importlib
import logging
from argparse import ArgumentParser

parser = ArgumentParser()
parser.add_argument("day")
parser.add_argument("-v", "--verbose")


def main():
    args = parser.parse_args()
    day = importlib.import_module(f"src.{args.day}")
    parsed_input = day.parse(f"inputs/{args.day}.txt")
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    print(f"Part 1: {day.part1(parsed_input)}")
    print(f"Part 2: {day.part2(parsed_input)}")

    print("Hello from advent-of-code-2025!")


if __name__ == "__main__":
    main()
