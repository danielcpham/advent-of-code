from unittest import TestCase

from src.day7 import parse, part1, part2


class TestDay7(TestCase):
    @classmethod
    def setUpClass(cls):
        cls.input = parse("tests/inputs/test_day7.txt")

    def test_part1(self):
        self.assertEqual(part1(self.input), 21)

    def test_part2(self):
        self.assertEqual(part2(self.input), 40)
