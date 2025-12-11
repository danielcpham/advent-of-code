from typing import NamedTuple
from unittest import TestCase

from src.day1 import parse_rotation, part1, part2, rotate


class TestDay1(TestCase):
    @classmethod
    def setUpClass(cls):
        with open("tests/inputs/test_day1.txt") as f:
            cls.rotations = f.readlines()

    def test_part1(self):
        self.assertEqual(part1(self.rotations), 3)

    def test_part2(self):
        self.assertEqual(part2(self.rotations), 6)

    def test_rotations(self):
        class SubCase(NamedTuple):
            start: int
            rotation: str
            end: int
            num_zeros: int

        cases = {
            "no rotation": SubCase(50, "R0", 50, 0),
            "rotate passed zero": SubCase(99, "R10", 9, 1),
            "rotate to zero": SubCase(50, "R50", 0, 1),
            "rotate from zero": SubCase(0, "L99", 99, 0),
            "many flips": SubCase(50, "L1000", 50, 10),
            "many flips to zero": SubCase(50, "R1050", 0, 11),
        }
        for name, case in cases.items():
            with self.subTest(msg=name):
                print(name)
                out = rotate(case.start, parse_rotation(case.rotation))
                self.assertEqual(out.end, case.end)
                self.assertEqual(out.num_zeros, case.num_zeros)
