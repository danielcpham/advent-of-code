from unittest import TestCase
from src.day2 import parse_ranges, part1, part2


class TestDay1(TestCase):
    @classmethod
    def setUpClass(cls):
        with open("tests/inputs/test_day2.txt") as f:
            input = f.readline()
            cls.ranges = parse_ranges(input)

    def test_part1(self):
        self.assertEqual(part1(self.ranges), 1227775554)

    def test_part2(self):
        self.assertEqual(part2(self.ranges), 4174379265)
