from src.lib import part1, part2, parse


def main():
    parsed_input = parse("input.txt")
    print(f"Part 1: {part1(parsed_input)}")
    print(f"Part 2: {part2(parsed_input)}")
    print("Hello from day3!")


if __name__ == "__main__":
    main()
