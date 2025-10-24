#!/usr/bin/env python3

def main():
    rules = []

    with open("./input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]

    for line in lines:
        s = line.split()

        if len(s) == 6:
            value, bot = int(s[1]), int(s[5])
            rules.append((value, bot))

        elif len(s) == 12:
            src = int(s[1])
            dst1 = int(s[6])
            dst2 = int(s[11])

            if s[5] == "output":
                dst1 = "o", dst1
            elif s[5] == "bot":
                dst1 = "b", dst1

            if s[10] == "output":
                dst2 = "o", dst2
            elif s[10] == "bot":
                dst2 = "b", dst2

            rules.append((src, dst1, dst2))

    print(*rules, sep="\n")


if __name__ == "__main__":
    main()
