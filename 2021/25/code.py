#!/usr/bin/python3


def show(file: list):

    for i in file:
        print("".join(i))
    print()


def moveHor(file: list) -> list:

    changed = False

    mi = len(file) - 1
    mj = len(file[0]) - 1

    result = []
    for i in range(0, mi + 1):
        result.append(file[i].copy())

    for i in range(0, mi + 1):
        for j in range(0, mj + 1):
            if file[i][j] == '<':
                if j > 0:
                    if file[i][j - 1] == '.':
                        result[i][j - 1] = '<'
                        result[i][j] = '.'
                        changed = True
                else:
                    if file[i][mj] == '.':
                        result[i][mj] = '<'
                        result[i][j] = '.'
                        changed = True
            if file[i][j] == '>':
                if j < mj:
                    if file[i][j + 1] == '.':
                        result[i][j + 1] = '>'
                        result[i][j] = '.'
                        changed = True
                else:
                    if file[i][0] == '.':
                        result[i][0] = '>'
                        result[i][j] = '.'
                        changed = True

    return result, changed


def moveVer(file: list) -> list:

    changed = False

    mi = len(file) - 1
    mj = len(file[0]) - 1

    result = []
    for i in range(0, mi + 1):
        result.append(file[i].copy())

    for j in range(0, mj + 1):
        for i in range(0, mi + 1):
            if file[i][j] == '^':
                if i > 0:
                    if file[i - 1][j] == '.':
                        result[i - 1][j] = '^'
                        result[i][j] = '.'
                        changed = True
                else:
                    if file[mi][j] == '.':
                        result[mi][j] = '^'
                        result[i][j] = '.'
                        changed = True
            if file[i][j] == 'v':
                if i < mi:
                    if file[i + 1][j] == '.':
                        result[i + 1][j] = 'v'
                        result[i][j] = '.'
                        changed = True
                else:
                    if file[0][j] == '.':
                        result[0][j] = 'v'
                        result[i][j] = '.'
                        changed = True

    return result, changed


if __name__ == '__main__':

    with open('input.txt', 'r') as f:
        file = f.read().split('\n')
        file.pop(len(file) - 1)
        file = [list(i) for i in file]

    counter = 0
    # show(file)
    while True:
        file, statusHor = moveHor(file)
        file, statusVer = moveVer(file)
        # show(file)
        counter += int(statusHor or statusVer)
        if not (statusHor or statusVer):
            print(f"Stopped after {counter + 1} steps")
            break

    with open('output.txt', 'w') as f:
        output = [''.join(i) for i in file]
        output = '\n'.join(output)
        f.write(output)
        f.write('\n')
