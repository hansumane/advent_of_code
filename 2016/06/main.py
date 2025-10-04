#!/usr/bin/env python3

from operator import itemgetter


def part12(lines):
    res1, res2 = [], []
    lines = [[line[i] for line in lines] for i in range(len(lines[0]))]
    for line in lines:
        letters = dict()
        for unique in set(line):
            letters[unique] = line.count(unique)
        s = sorted(letters.items(), key=itemgetter(1), reverse=True)
        res1.append(s[0][0])
        res2.append(s[-1][0])
    return "".join(res1), "".join(res2)


def main():
    with open("./input.txt", "r") as f:
        lines = [list(line.strip()) for line in f.readlines()]
    print(part12(lines))


if __name__ == "__main__":
    main()
