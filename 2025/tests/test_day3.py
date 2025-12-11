from unittest import TestCase
from src.day3 import part1, part2, parse


class TestDay3(TestCase):
    @classmethod
    def setUpClass(cls):
        cls.input = parse("tests/inputs/test_day3.txt")

    def test_part1(self):
        self.assertEqual(part1(self.input), 357)

    def test_part2(self):
        self.assertEqual(part2(self.input), 3121910778619)
