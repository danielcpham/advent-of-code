from collections import defaultdict
from functools import reduce
from operator import add, mul


def parse(filename: str) -> list[str]:
    with open(filename) as f:
        input = f.read().splitlines()
    return input


def part1(input: list[str]) -> int:
    operations = input[-1].split()
    operations = [mul if op == "*" else add for op in operations]
    operands = defaultdict(list)
    for line in input[0:-1]:
        for i, num in enumerate([int(n) for n in line.split()]):
            operands[i].append(num)
    outputs = []
    for i, nums in operands.items():
        op = operations[i]
        outputs.append(reduce(op, nums, 1 if op == mul else 0))
    return sum(outputs)


def part2(input) -> int:
    operations = input[-1].split()
    op_indexes = []
    operations = []
    for i, c in enumerate(input[-1]):
        if c in ["*", "+"]:
            op_indexes.append(i)
            if c == "*":
                operations.append(mul)
            else:
                operations.append(add)

    def split_nums_at_indexes(line: str):
        return [
            line[i : j - 1]
            for i, j in zip(op_indexes, op_indexes[1:] + [len(line) + 1])
        ]

    operands: defaultdict[list[str]] = defaultdict(list)
    for line in input[0:-1]:
        for i, n in enumerate(split_nums_at_indexes(line)):
            operands[i].append(n)
    for idx in operands.keys():
        ns = operands[idx]
        # pad operands
        max_length = max(len(n) for n in ns)
        ns = [n.rjust(max_length, "0").replace(" ", "0") for n in ns]
        # read nums by column
        new_operands = []
        for i in range(max_length):
            # Left justified spaces become extra 0s on the right, which
            # need to be removed
            new_operand = int("".join([val[i] for val in ns]).rstrip("0"))
            new_operands.append(new_operand)
        operands[idx] = new_operands
    outputs = []
    for i, nums in operands.items():
        op = operations[i]
        outputs.append(reduce(op, nums, 1 if op == mul else 0))
    return sum(outputs)
