#!/usr/bin/env python3

import re


def part1(ranges: list[tuple[int, int]]) -> int:
    """
    I think this is pretty OK.
    """

    res = 0

    for start, end in ranges:
        for n in range(start, end + 1):
            s = str(n)
            if s[:len(s) // 2] == s[len(s) // 2:]:
                res += n

    return res


def part2(ranges: list[tuple[int, int]]) -> int:
    """
    Regex is a sweet candy.
    This is slow though.
    """

    rx = re.compile(r'^(\d+)\1+$')
    res = 0

    for start, end in ranges:
        for n in range(start, end + 1):
            if len(re.findall(rx, str(n))) > 0:
                res += n

    return res


def main():
    with open("./input.txt", "r") as f:
        ranges = [tuple(map(int, pair.split("-"))) for pair in f.readline().strip().split(",")]

    print(part1(ranges))  # pyright: ignore
    print(part2(ranges))  # pyright: ignore


if __name__ == "__main__":
    main()
