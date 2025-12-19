import importlib
import logging
import os
from argparse import ArgumentParser

import dotenv
import requests
from jinja2 import Environment, PackageLoader, select_autoescape

parent_parser = ArgumentParser(add_help=False)
parent_parser.add_argument("--day", type=int, help="Day number")
parent_parser.add_argument("-v", "--verbose")

parser = ArgumentParser(prog="Advent of Code 2025")
subparsers = parser.add_subparsers()


def run(args):
    day = importlib.import_module(f"src.day{args.day}")
    parsed_input = day.parse(
        f"inputs/day{args.day}.txt"
        if not args.test
        else f"tests/inputs/test_day{args.day}.txt"
    )
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    if not args.part or args.part == 1:
        print(f"Part 1: {day.part1(parsed_input)}")
    if not args.part or args.part == 2:
        print(f"Part 2: {day.part2(parsed_input)}")


def download(args):
    url = f"https://adventofcode.com/2025/day/{args.day}/input"
    input_file = f"inputs/day{args.day}.txt"
    env = Environment(loader=PackageLoader("src"), autoescape=select_autoescape())

    src_filename = f"{os.getcwd()}/src/day{args.day}.py"
    src_template = env.get_template("dayx.py.jinja")
    test_filename = f"{os.getcwd()}/tests/test_day{args.day}.py"
    test_template = env.get_template("test_dayx.py.jinja")
    test_input = f"{os.getcwd()}/tests/inputs/test_day{args.day}.txt"

    if args.dry_run:
        print(f"Input URL: {url}")
        print(src_filename)
        print(src_template.render(day=args.day))
        print(test_filename)
        print(test_template.render(day=args.day))
    else:
        if not os.getenv("SESSION"):
            print(
                "Please set the SESSION environment variable with your Advent of Code session cookie."
            )
            return
        response = requests.get(url, cookies={"session": os.getenv("SESSION")})
        if response.status_code == 200:
            with open(input_file, "w") as f:
                f.write(response.text)
            print(f"Input for day {args.day} downloaded successfully.")
            with open(src_filename, "w") as f:
                f.write(src_template.render(day=args.day))
            print("Created source file:", src_filename)
            with open(test_filename, "w") as f:
                f.write(test_template.render(day=args.day))
            print("Created test file:", test_filename)
            with open(test_input, "w") as f:
                f.write("")
            print("Created test input file:", test_input)
        else:
            print(
                f"Failed to download input for day {args.day}. Status code: {response.status_code}"
            )


run_parser = subparsers.add_parser(
    "run", parents=[parent_parser], help="Run the solution"
)
run_parser.add_argument(
    "--part",
    required=False,
    type=int,
    choices=[1, 2],
    help="Part of the day's challenge to run",
)
run_parser.add_argument("--test", action="store_true", help="Run with test input")
run_parser.set_defaults(func=run)

download_parser = subparsers.add_parser(
    "download", parents=[parent_parser], help="Download the input"
)
download_parser.add_argument(
    "--dry-run", action="store_true", help="Print the URL without downloading"
)
download_parser.set_defaults(func=download)


def main():
    print("Hello from advent-of-code-2025!")
    dotenv.load_dotenv()
    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
