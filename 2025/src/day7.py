from collections import defaultdict
from dataclasses import dataclass
from typing import Iterable, NamedTuple


class Point(NamedTuple):
    row: int
    col: int

    def __repr__(self) -> str:
        return f"({self.row}, {self.col})"


@dataclass
class Input:
    width: int
    height: int
    start: Point
    splitters: set[Point]

    def show_input(self, beams: Iterable[Point] | None = None) -> None:
        lines = [["." for _ in range(self.width)] for _ in range(self.height)]
        lines[self.start.row][self.start.col] = "S"
        for i, j in self.splitters:
            lines[i][j] = "^"
        if beams:
            for i, j in beams:
                lines[i][j] = "|"
        print("\n".join(" ".join(line) for line in lines))


def parse(filename: str) -> Input:
    with open(filename) as f:
        input = f.read().splitlines()
    start = None
    splitters = set()
    for i, line in enumerate(input):
        for j, char in enumerate(line):
            if char == "S":
                start = Point(i, j)
            elif char == "^":
                splitters.add(Point(i, j))
    input_data = Input(
        width=len(input[0]),
        height=len(input),
        start=start,
        splitters=splitters,
    )
    return input_data


def part1(input: Input) -> int:
    start = input.start
    splits = 0
    all_beams = set()
    beams = set([input.start])
    for i in range(start.row, input.height - 1):
        new_beams = []
        for beam_row, beam_col in beams:
            print(beam_row, beam_col)
            # If empty, move down
            if Point(i + 1, beam_col) not in input.splitters:
                new_beams.append(Point(i + 1, beam_col))
            else:
                left_split = Point(i + 1, max(beam_col - 1, 0))
                right_split = Point(i + 1, min(beam_col + 1, input.width))
                new_beams.append(left_split)
                new_beams.append(right_split)
                splits += 1
        beams = set(new_beams)
        all_beams |= beams
        print("\n" * 2)
        input.show_input(all_beams)
    return splits


def part2(input) -> int:
    start = input.start
    num_paths: defaultdict[Point] = defaultdict(int)
    num_paths[input.start] += 1
    beams = set([input.start])
    for i in range(start.row, input.height - 1):
        new_beams = set()
        for beam_row, beam_col in beams:
            if Point(i + 1, beam_col) not in input.splitters:
                new_beams.add(Point(i + 1, beam_col))
                # Number of paths stays the same
                num_paths[Point(i + 1, beam_col)] += num_paths[(beam_row, beam_col)]
            # If splitter, split
            else:
                left_split = Point(i + 1, max(beam_col - 1, 0))
                right_split = Point(i + 1, min(beam_col + 1, input.width))
                new_beams.add(left_split)
                # Total number of paths is sum of paths continuing the beam
                # and paths split from adjacent beams
                num_paths[left_split] += num_paths[(beam_row, beam_col)]
                new_beams.add(right_split)
                num_paths[right_split] += num_paths[(beam_row, beam_col)]
        beams = new_beams
    return sum(val for p, val in num_paths.items() if p.row == input.height - 1)
