#!/usr/bin/env python3

def part1(batteries: list[list[int]]):
    res = 0

    for bat in batteries:
        max_joltage = 0
        for i in range(0, len(bat) - 1):
            for j in range(i + 1, len(bat)):
                joltage = 10 * bat[i] + bat[j]
                if joltage > max_joltage:
                    max_joltage = joltage
        res += max_joltage

    return res


def part2(batteries: list[list[int]]):
    """
    Too hard... ;T_T;
    """
    pass


def main():
    with open("./input.txt", "r") as f:
        lines = [l.strip() for l in f.readlines()]

    batteries = [list(map(int, list(l))) for l in lines]

    print(f"{part1(batteries) = }")


if __name__ == "__main__":
    main()
