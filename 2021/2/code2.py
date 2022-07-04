#!/usr/bin/python3
# 2021/2/code2.py


if __name__ == '__main__':
    with open('input.txt', 'r') as f:
        file = f.read().split('\n')
        file.pop(len(file) - 1)
        for i in range(len(file)):
            file[i] = file[i].split()
            file[i][1] = int(file[i][1])
        horizontal, depth, aim = 0, 0, 0
        for i in file:
            if i[0] == 'forward':
                horizontal += i[1]
                depth += i[1] * aim
            elif i[0] == 'up':
                aim -= i[1]
            elif i[0] == 'down':
                aim += i[1]
            else:
                raise TypeError('key word is not forward, up, down')
                break
        print(f'{horizontal} * {depth} = {horizontal * depth}')
