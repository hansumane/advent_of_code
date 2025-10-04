#!/usr/bin/env python3

PAD1 = [["1", "2", "3"],
        ["4", "5", "6"],
        ["7", "8", "9"]]

def adds1(v, n):
    if n < 0 and v <= 0 or n > 0 and v >= 2:
        return v
    return v + n

def part1(lines):
    res = ""
    i, j = 1, 1
    for line in lines:
        for d in line:
            if d == "U":
                i = adds1(i, -1)
            elif d == "D":
                i = adds1(i, +1)
            elif d == "R":
                j = adds1(j, +1)
            elif d == "L":
                j = adds1(j, -1)
        res += PAD1[i][j]
    return res


PAD2 = [[" ", " ", "1", " ", " "],
        [" ", "2", "3", "4", " "],
        ["5", "6", "7", "8", "9"],
        [" ", "A", "B", "C", " "],
        [" ", " ", "D", " ", " "]]

def adds2(i, j, n, mode):
    if mode == "i":
        if n < 0 and i <= 0 or n > 0 and i >= 4 or PAD2[i + n][j] == " ":
            return i
        return i + n
    elif mode == "j":
        if n < 0 and j <= 0 or n > 0 and j >= 4 or PAD2[i][j + n] == " ":
            return j
        return j + n

def part2(lines):
    res = ""
    i, j = 2, 0
    for line in lines:
        for d in line:
            if d == "U":
                i = adds2(i, j, -1, "i")
            elif d == "D":
                i = adds2(i, j, +1, "i")
            elif d == "R":
                j = adds2(i, j, +1, "j")
            elif d == "L":
                j = adds2(i, j, -1, "j")
        res += PAD2[i][j]
    return res


def main():
    with open("./input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
    print("1:", part1(lines))
    print("2:", part2(lines))


if __name__ == "__main__":
    main()
