# Correct answers:
# 1: 1787
# 2: 440640

# I don't like the code :<


def first():

    counter = 2 * size + 2 * (size - 2)

    for i in range(1, size - 1):

        for j in range(1, size - 1):

            value = lines[i][j]

            for n in range(0, j):
                if lines[i][n] >= value:
                    visible = False
                    break
            else:
                visible = True

            for n in range(j + 1, size):
                if lines[i][n] >= value:
                    visible = False or visible
                    break
            else:
                visible = True

            for n in range(0, i):
                if lines[n][j] >= value:
                    visible = False or visible
                    break
            else:
                visible = True

            for n in range(i + 1, size):
                if lines[n][j] >= value:
                    visible = False or visible
                    break
            else:
                visible = True

            if visible:
                counter += 1

    return counter


def find_scenic(i, j):

    value = lines[i][j]
    n, w, e, s = 0, 0, 0, 0

    for jn in range(j - 1, -1, -1):
        n += 1
        if lines[i][jn] >= value:
            break

    for js in range(j + 1, size):
        s += 1
        if lines[i][js] >= value:
            break

    for iw in range(i - 1, -1, -1):
        w += 1
        if lines[iw][j] >= value:
            break

    for ie in range(i + 1, size):
        e += 1
        if lines[ie][j] >= value:
            break

    return n * w * e * s


def second():
    m = 0
    for i in range(1, size - 1):
        for j in range(1, size - 1):
            s = find_scenic(i, j)
            if s > m:
                m = s
    return m


with open('input.txt', 'r') as f:
    lines = [list(map(int, line.strip())) for line in f.readlines()]
size = len(lines)
print(first())
print(second())
