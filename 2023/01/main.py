#!/usr/bin/env python3
from sys import argv

# don't close files, it's ok ;0_0;

# first

print('1)', sum(map(
    lambda v: v[0] * 10 + v[-1],
    (list(map(int, filter(lambda v: v.isdecimal(), line)))
     for line in open(argv[1], "r").readlines())
    )))

# second

l = [*map(str, range(10)), 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
def getval(ss):
    i = l.index(ss)
    return i - 9 if i >= 10 else i

res = 0
for line in open(argv[1], 'r').readlines():
    ld = []
    for ss in l:
        f = -1
        while (f := line.find(ss, f + 1)) >= 0:
            ld.append((f, ss))
    ld.sort(key=lambda v: v[0])
    a = getval(ld[0][1])
    b = getval(ld[-1][1])
    res += a * 10 + b
print('2)', res)
