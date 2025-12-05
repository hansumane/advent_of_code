#!/usr/bin/env python3

def get_removable(rolls: list[list[str]]) -> list[tuple[int, int]]:
    removable = []

    for i in range(len(rolls)):
        for j in range(len(rolls[i])):
            if rolls[i][j] != '@':
                continue

            ctr = \
                int(i > 0 and j > 0                 and rolls[i - 1][j - 1] == '@') + \
                int(i > 0 and                           rolls[i - 1][  j  ] == '@') + \
                int(i > 0 and j < len(rolls[i]) - 1 and rolls[i - 1][j + 1] == '@') + \
                \
                int(j > 0                 and rolls[i][j - 1] == '@') + \
                int(j < len(rolls[i]) - 1 and rolls[i][j + 1] == '@') + \
                \
                int(i < len(rolls) - 1 and j > 0                 and rolls[i + 1][j - 1] == '@') + \
                int(i < len(rolls) - 1                           and rolls[i + 1][  j  ] == '@') + \
                int(i < len(rolls) - 1 and j < len(rolls[i]) - 1 and rolls[i + 1][j + 1] == '@')

            if ctr < 4:
                removable.append((i, j))

    return removable


def part1(rolls: list[list[str]]) -> int:
    return len(get_removable(rolls))


def part2(rolls: list[list[str]]) -> int:
    res = 0

    while len(removable := get_removable(rolls)) > 0:
        for i, j in removable:
            rolls[i][j] = 'x'
            res += 1

    return res


def main():
    with open("./input.txt", "r") as f:
        rolls = [list(l.strip()) for l in f.readlines()]
    print(f"{part1(rolls) = }")
    print(f"{part2(rolls) = }")


if __name__ == "__main__":
    main()
