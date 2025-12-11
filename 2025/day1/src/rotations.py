

from operator import add, sub
from typing import Literal, NamedTuple

class Rotation(NamedTuple):
    dir: Literal["L", "R"]
    distance: int
    
    @property
    def val(self):
        return -self.distance if self.dir == "L" else self.distance
    
    @property
    def op(self):
        if self.dir=="L":
            return sub
        else:
            return add

    def __str__(self):
        return ("-" if self.dir == "L" else "+") + str(self.distance)
    
    
class RotationOutput(NamedTuple):
    end: int 
    """
    Result of rotation.
    """
    num_zeros: int
    """
    number of times zero is passed
    """
    


def rotate(start: int, rotation: Rotation) -> RotationOutput:
    print(start, rotation.dir, rotation.distance)
   
    distance = rotation.distance
    num_flips, leftover = divmod(distance, 100)
    end = rotation.op(start, leftover)
    num_zeros = num_flips
    print(f"{rotation.distance=}: {num_flips=}, {leftover=}, {end%100=}")
    if (start != 0 and not (0 < end < 100)):
        num_zeros +=1 
    end = end % 100

    return RotationOutput(
        end=end,
        num_zeros=num_zeros
    )
    
def parse_rotation(rotation_str: str) -> Rotation:
    return Rotation(rotation_str[0], int(rotation_str[1:]))

def parse_rotations(rotations) -> list[Rotation]:
	return [
		parse_rotation(rotation)
		for rotation in rotations
	]


def part1(rotations:list[Rotation]) -> int:
    position = 50
    rotations = parse_rotations(rotations)
    num_zeros = 0
    for rotation in rotations:
        if rotation.dir == "L":
            position = (position - rotation.distance) % 100
        else:
            position = (position + rotation.distance) % 100
        if position == 0:
            num_zeros +=1
    return num_zeros

def part2(rotations):
    position = 50
    rotations = parse_rotations(rotations)
    num_flips = 0
    for rotation in rotations:
        new_position, num_zeros = rotate(position, rotation)
        position = new_position
        num_flips += num_zeros
    print(num_flips)
    return num_flips
            
    
    
    
        