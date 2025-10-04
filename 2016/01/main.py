#!/usr/bin/env python3

faces = ["N", "E", "S", "W"]


def next_face(face, rl):
    if rl == "R":
        return faces[(faces.index(face) + 1) % len(faces)]
    elif rl == "L":
        return faces[(faces.index(face) - 1) % len(faces)]
    else:
        assert False, "Unreachable"


def main():
    cur_face = "N"
    i, j = 0, 0

    with open("./input.txt", "r") as f:
        directions = f.readline().strip().split(", ")
    directions = [(d[:1], int(d[1:])) for d in directions]
    all_visited = [(0, 0)]
    second_found = False

    for (rl, dist) in directions:
        cur_face = next_face(cur_face, rl)
        if cur_face == "N":
            ni = i + dist
            nj = j
        elif cur_face == "E":
            ni = i
            nj = j + dist
        elif cur_face == "S":
            ni = i - dist
            nj = j
        elif cur_face == "W":
            ni = i
            nj = j - dist
        else:
            assert False, "Unreachable"

        if not second_found:
            if ni == i:
                for s in range(min(j, nj), max(j, nj) + 1):
                    if (i, s) == (i, j):
                        continue
                    if (i, s) in all_visited:
                        print("2:", abs(i) + abs(s))
                        second_found = True
                    all_visited.append((i, s))
            elif nj == j:
                for s in range(min(i, ni), max(i, ni) + 1):
                    if (s, j) == (i, j):
                        continue
                    if (s, j) in all_visited:
                        print("2:", abs(s) + abs(j))
                        second_found = True
                    all_visited.append((s, j))
            else:
                assert False, "Unreachable"

        i, j = ni, nj

    print("1:", abs(i) + abs(j))


if __name__ == "__main__":
    main()
