#!/usr/bin/python3
# 2021/03/code1.py


if __name__ == '__main__':

    with open('input.txt', 'r') as f:

        file = f.read().split('\n')
        file.pop(len(file) - 1)

        gamma_rate, epsilon_rate = [], []

        for i in range(len(file[0])):
            count0, count1 = 0, 0
            for j in file:
                if j[i] == '0':
                    count0 += 1
                elif j[i] == '1':
                    count1 += 1
            if count0 > count1:
                gamma_rate.append('0')
                epsilon_rate.append('1')
            else:
                gamma_rate.append('1')
                epsilon_rate.append('0')

        gamma_rate = int(''.join(gamma_rate), 2)
        epsilon_rate = int(''.join(epsilon_rate), 2)
        print(gamma_rate * epsilon_rate)
