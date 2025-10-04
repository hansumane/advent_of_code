#!/usr/bin/env python3

from operator import itemgetter


def part1(rooms):
    res = 0
    for (words, number, csum) in rooms:
        joined = "".join(words)
        alphabet = [(sym, joined.count(sym)) for sym in set(joined)]
        alphabet.sort(key=itemgetter(0))
        alphabet.sort(key=itemgetter(1), reverse=True)
        if csum == "".join(v[0] for v in alphabet[:5]):
            res += number
    return res


def rotate_words(words, by):
    res = []
    from string import ascii_lowercase as alphabet
    for word in words:
        rotated_word = ""
        for sym in word:
            rotated_word += alphabet[(alphabet.index(sym) + by) % len(alphabet)]
        res.append(rotated_word)
    return res


def part2(rooms):
    for (words, number, _) in rooms:
        rotated_words = rotate_words(words, number)
        if " ".join(rotated_words) == "northpole object storage":
            return number


def main():
    rooms = []
    with open("./input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
        for line in lines:
            obi, cbi = line.index("["), line.index("]")
            data = line[:obi].split("-")
            csum = line[obi + 1:cbi]
            rooms.append((data[:-1], int(data[-1]), csum))

    print(part1(rooms))
    print(part2(rooms))


if __name__ == "__main__":
    main()
