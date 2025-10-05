#!/usr/bin/env python3

from operator import itemgetter


def part1(ips):
    res = 0
    for ip in ips:
        valid = False
        ip = sorted(ip, key=itemgetter(0))  # "" < "b"
        for (b, syms) in ip:
            for i in range(len(syms) - 3):
                t = syms[i:i + 4]
                if t[0] != t[1] and f"{t[0]}{t[1]}" == f"{t[3]}{t[2]}":
                    if b == "":
                        valid = True
                    elif b == "b":
                        valid = False
                    break
        if valid:
            res += 1
    return res


def part2(ips):
    res = 0
    for ip in ips:
        valid, patterns = False, []
        ip = sorted(ip, key=itemgetter(0))  # "" < "b"
        for (b, syms) in ip:
            for i in range(len(syms) - 2):
                t = syms[i:i + 3]
                if t[0] != t[1] and f"{t[0]}{t[1]}" == f"{t[2]}{t[1]}":
                    if b == "":
                        patterns.append(t)
                    elif b == "b":
                        tt = f"{t[1]}{t[0]}{t[1]}"
                        if tt in patterns:
                            valid = True
                            break
        if valid:
            res += 1
    return res


def main():
    with open("./input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]

    ips = []
    for line in lines:
        res = []  # [("b" | "", str)]
        v = ""
        for sym in line:
            if sym == "[":
                res.append(("", v))
                v = ""
            elif sym == "]":
                res.append(("b", v))
                v = ""
            else:
                v = v + sym
        res.append(("", v))
        ips.append(res)

    print(part1(ips))
    print(part2(ips))


if __name__ == "__main__":
    main()
