#!/usr/bin/python3


def parse(file: list) -> dict:

    numbers = list(map(int, file[0].split(',')))
    file.pop(0)

    while '' in file:
        file.pop(file.index(''))

    i, boards = 0, list()
    while i < len(file):
        try:
            boards[i//5].append(list(map(int, file[i].split())))
        except IndexError:
            boards.append([])
            boards[i//5].append(list(map(int, file[i].split())))
        i += 1

    for i in range(len(boards)):
        for j in range(len(boards[i])):
            for k in range(len(boards[i][j])):
                boards[i][j][k] = [boards[i][j][k], 0]

    return numbers, boards


def check(boards: list) -> int:

    for i in range(len(boards)):
        for j in range(len(boards[i])):
            count = 0
            for k in range(len(boards[i][j])):
                count += boards[i][j][k][1]
            if count == 5:
                return i

    return -1


def usum(board: list) -> int:

    result = 0

    for i in range(len(board)):
        for j in range(len(board[i])):
            if board[i][j][1] == 0:
                result += board[i][j][0]

    return result


if __name__ == '__main__':

    with open('input.txt', 'r') as f:
        file = f.read().split('\n')

    numbers, boards = parse(file)

    for number in numbers:
        for i in range(len(boards)):
            for j in range(len(boards[i])):
                for k in range(len(boards[i][j])):
                    if boards[i][j][k][0] == number:
                        boards[i][j][k][1] = 1
        answer = check(boards)
        if answer >= 0:
            result = number * usum(boards[answer])
            print(result)
            break
