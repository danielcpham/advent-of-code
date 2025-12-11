from dataclasses import dataclass
import itertools
import copy
import logging

@dataclass
class Map:
    num_rows: int
    num_cols: int
    paper_locations: set[(int, int)]
    map_lines: list[str]
    
    def __init__(self, map_lines):
        self.map_lines=map_lines
        self.num_rows=len(map_lines)
        self.num_cols=len(map_lines[0])
        self.paper_locations=set([])
    
    def get_reachable_spots(self) -> list[tuple[int, int]]:
        reachable_spots = []
        for (row, col) in self.paper_locations:
            adjacent_locations = {
                (i, j) for i, j in itertools.product([row - 1, row, row +1],[col - 1, col, col + 1])
                if 0<=i<self.num_rows and 0<=j<self.num_cols and not (i,j) == (row, col)
            }
            # print((row, col), adjacent_locations.intersection(map.paper_locations))
            if len(adjacent_locations.intersection(self.paper_locations)) < 4:
                reachable_spots.append((row, col))
        return reachable_spots
    
    def mark_spots(self, locations: list[tuple[int, int]]) -> str:
        map_lines = copy.deepcopy(self.map_lines)
        for row, col in locations:
            map_lines[row]= map_lines[row][:col] + "x" + map_lines[row][col+1:]
        return "\n".join(map_lines)
    
    def remove_paper(self, locations: list[tuple[int, int]]) -> None:
        for row, col in locations:
            self.map_lines[row] = self.map_lines[row][:col] + "." + self.map_lines[row][col+1:]
            self.paper_locations.remove((row, col))
            
    def __str__(self):
        return "\n".join(self.map_lines)
    
    
def parse(file: str) -> list[str]:
    with open(file) as f:
        input = f.read().splitlines()
    return input

def read_map(map_lines: list[str]) -> Map:
    map = Map(map_lines=map_lines)
    for i, row in enumerate(map_lines):
        for j, val in enumerate(row):
            if val == "@":
                map.paper_locations.add((i,j))
    return map
        

def part1(map_lines: list[str]):
    map = read_map(map_lines)
    reachable_spots = map.get_reachable_spots()
    map.mark_spots(reachable_spots)
    return len(reachable_spots)


def part2(map_lines: list[str]) -> int:
    map = read_map(map_lines)
    num_spots_accessed = 0
    reachable_spots = map.get_reachable_spots()
    while len(reachable_spots) > 0:
        num_spots_accessed += len(reachable_spots)
        logging.debug("\n"+str(map))
        map.remove_paper(reachable_spots)    
        logging.debug("\n"+str(map))
        reachable_spots = map.get_reachable_spots()
    return num_spots_accessed