#!/usr/bin/env python3

def part1(line):
    i, res = 0, ""
    while i < len(line):
        if line[i] == "(":
            i += 1
            part = ""
            while line[i] != ")":
                part = part + line[i]
                i += 1
            i += 1
            count, repeat = map(int, part.split("x"))
            for _ in range(repeat):
                res += line[i:i + count]
            i += count
        else:
            res += line[i]
            i += 1
    return len(res)


def part2(line):
    i, res = 0, 0
    while i < len(line):
        if line[i] == "(":
            b = i + 1
            while line[i] != ")":
                i += 1
            count, repeat = map(int, line[b:i].split("x"))
            res += repeat * part2(line[i + 1:i + 1 + count])
            i = i + 1 + count
        else:
            i += 1
            res += 1
    return res


def main():
    with open("./input.txt", "r") as f:
        line = "".join(f.readline().strip().split())
    print(part1(line))
    print(part2(line))


if __name__ == "__main__":
    main()
