import heapq
import math
from dataclasses import dataclass
from functools import reduce
from itertools import combinations
from operator import mul
from typing import NamedTuple, Self


class Point(NamedTuple):
    x: int
    y: int
    z: int

    def __repr__(self) -> str:
        return f"{self.x},{self.y},{self.z}"

    def distance(self, other: Self) -> float:
        return math.sqrt(
            (self.x - other.x) ** 2 + (self.y - other.y) ** 2 + (self.z - other.z) ** 2
        )


@dataclass
class Input:
    boxes: list[Point]


def parse(filename: str) -> Input:
    with open(filename) as f:
        input = f.read().splitlines()
    boxes = []
    for line in input:
        x, y, z = map(int, line.split(","))
        boxes.append(Point(x, y, z))
    return Input(boxes=boxes)


def part1(input: Input) -> int:
    distances = [(p1.distance(p2), p1, p2) for p1, p2 in combinations(input.boxes, 2)]
    heapq.heapify(distances)
    connection_map = {point: (point,) for point in input.boxes}

    for _ in range(1000):
        _, p1, p2 = heapq.heappop(distances)
        print(p1, p2)
        p1_connection = connection_map[p1]
        p2_connection = connection_map[p2]
        if p1_connection != p2_connection:
            new_connection = (*p1_connection, *p2_connection)
            for p in new_connection:
                connection_map[p] = new_connection
        else:
            print("Already connected:", p1_connection)
    connections_length_sorted = sorted(
        [(len(conn), conn) for conn in set(connection_map.values())],
        key=lambda x: x[0],
        reverse=True,
    )
    product_of_lengths = reduce(
        mul, [length for length, _ in connections_length_sorted[:3]], 1
    )
    return product_of_lengths


def part2(input: Input) -> int:
    distances = [(p1.distance(p2), p1, p2) for p1, p2 in combinations(input.boxes, 2)]
    heapq.heapify(distances)
    connection_map = {point: (point,) for point in input.boxes}
    connections = set(val for val in connection_map.values())
    last_connection = None
    while len(connections) > 1:
        _, p1, p2 = heapq.heappop(distances)
        p1_connection = connection_map[p1]
        p2_connection = connection_map[p2]
        if p1_connection != p2_connection:
            if p1_connection != p2_connection:
                new_connection = (*p1_connection, *p2_connection)
                for p in new_connection:
                    connection_map[p] = new_connection
                connections.discard(p1_connection)
                connections.discard(p2_connection)
                connections.add(new_connection)
                last_connection = (p1, p2)
    print(last_connection)
    return last_connection[0].x * last_connection[1].x
