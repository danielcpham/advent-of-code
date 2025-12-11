from unittest import TestCase

from src.day2 import parse, part1, part2


class TestDay1(TestCase):
    @classmethod
    def setUpClass(cls):
        with open("tests/inputs/test_day2.txt") as f:
            cls.ranges = parse("tests/inputs/test_day2.txt")

    def test_part1(self):
        self.assertEqual(part1(self.ranges), 1227775554)

    def test_part2(self):
        self.assertEqual(part2(self.ranges), 4174379265)
