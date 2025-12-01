#!/usr/bin/env python3

def part1(pairs: list[tuple[str, int]]):
    """
    This is OK.
    """

    ctr, cur = 0, 50

    for lr, n in pairs:
        if lr == "L":
            cur = (cur - n) % 100
        elif lr == "R":
            cur = (cur + n) % 100

        if cur == 0:
            ctr += 1

    return ctr


def part2(pairs: list[tuple[str, int]]):
    """
    This is weird, but it works, and it works fast enough.
    """

    ctr, cur = 0, 50

    for lr, n in pairs:
        if lr == "L":
            while n > 0:
                cur, n = cur - 1, n - 1

                if cur == 0:
                    ctr += 1
                if cur < 0:
                    cur += 100
        elif lr == "R":
            while n > 0:
                cur, n = cur + 1, n - 1

                if cur > 99:
                    cur -= 100
                if cur == 0:
                    ctr += 1

    return ctr


def main():
    pairs = []
    with open("./input.txt", "r") as f:
        for line in f.readlines():
            l = line.strip()
            pairs.append((l[0], int(l[1:])))

    print(part1(pairs))
    print(part2(pairs))


if __name__ == "__main__":
    main()
