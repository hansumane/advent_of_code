#!/usr/bin/env python3

def part1(lines):
    ctr = 0
    for values in lines:
        vs = sorted(values)
        if vs[0] + vs[1] > vs[2]:
            ctr += 1
    return ctr


def part2(lines):
    values = [vs[i] for i in range(3) for vs in lines]
    lines = [values[i:i + 3] for i in range(0, len(values), 3)]
    return part1(lines)


def main():
    with open("./input.txt", "r") as f:
        lines = [tuple(map(int, line.strip().split())) for line in f.readlines()]
    print(part1(lines))
    print(part2(lines))


if __name__ == "__main__":
    main()
