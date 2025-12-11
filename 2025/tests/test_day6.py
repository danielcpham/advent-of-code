from unittest import TestCase

from src.day6 import parse, part1, part2


class TestDay6(TestCase):
    @classmethod
    def setUpClass(cls):
        cls.input = parse("tests/inputs/test_day6.txt")

    def test_part1(self):
        self.assertEqual(part1(self.input), 4277556)

    def test_part2(self):
        self.assertEqual(part2(self.input), 3263827)
