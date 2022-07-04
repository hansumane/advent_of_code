#!/usr/bin/python3
# 2021/1/code2.py


if __name__ == '__main__':
    with open('input.txt', 'r') as f:
        file = f.read().split('\n')
        file.pop(len(file) - 1)
        file = list(map(int, file))
        counter = 0
        for i in range(1, len(file) - 2):
            if file[i + 2] > file[i - 1]:
                counter += 1
        print(counter)
