from src.rotations import part1, part2


def main():
    with open("input.txt") as f:
        rotations = f.readlines()
        part1(rotations)
        part2(rotations)
    print("Hello from day1!")


if __name__ == "__main__":
    main()
