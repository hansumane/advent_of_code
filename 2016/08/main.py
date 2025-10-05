#!/usr/bin/env python3

def rotate_column(display, n, by):
    for _ in range(by):
        v = display[n][-1]
        for i in range(49, 0, -1):
            display[n][i] = display[n][i - 1]
        display[n][0] = v


def rotate_row(display, n, by):
    for _ in range(by):
        v = display[-1][n]
        for i in range(5, 0, -1):
            display[i][n] = display[i - 1][n]
        display[0][n] = v


def part12(lines):
    display = [["." for _ in range(50)] for _ in range(6)]
    for line in lines:
        splitted = line.split()
        if "rect" in line:
            a, b = tuple(map(int, splitted[1].split("x")))
            for i in range(b):
                for j in range(a):
                    display[i][j] = "#"
        elif "rotate" in line:
            what, which = splitted[2].split("=")  # "x" | "y" , "int"
            which = int(which)
            by = int(splitted[4])
            if what == "x":
                rotate_row(display, which, by)
            elif what == "y":
                rotate_column(display, which, by)

    return (sum(row.count("#") for row in display),
            "\n".join("".join(row) for row in display))


def main():
    with open("./input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
    print(*part12(lines), sep="\n")


if __name__ == "__main__":
    main()
