#!/usr/bin/env python3
from math import prod
from sys import argv

# don't close files, it's ok ;0_0;

# first

print("1)", sum(map(lambda v: v[0], filter(
    lambda v: v[1],
    [(i, all(all((int(cubes.split()[0]) <=
                  {"red": 12, "green": 13, "blue": 14}[cubes.split()[1]])
                 for cubes in game.split(", "))
             for game in line.strip().split(": ")[1].split("; ")))
     for i, line in enumerate(open(argv[1], "r").readlines(), start=1)]
))))

# second

res2 = 0
for line in (line.strip() for line in open(argv[1], "r").readlines()):
    res = {"red": 0, "green": 0, "blue": 0}
    cubes = [[cube.split() for cube in game.split(", ")]
             for game in line.strip().split(": ")[1].split("; ")]
    for cube in cubes:
        for v, k in cube:
            if res[k] < (v := int(v)):
                res[k] = v
    res2 += prod(map(lambda v: v[1], res.items()))
print("2)", res2)
