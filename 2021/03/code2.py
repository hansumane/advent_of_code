#!/usr/bin/python3
# 2021/03/code2.py


if __name__ == '__main__':

    with open('input.txt', 'r') as f:

        file = f.read().split('\n')
        file.pop(len(file) - 1)

        oxygen = [file.copy()]
        for i in range(len(file[0])):
            count0, count1, results = 0, 0, []
            for j in oxygen[i]:
                if j[i] == '0':
                    count0 += 1
                elif j[i] == '1':
                    count1 += 1
            if count1 >= count0:
                dominant = '1'
            else:
                dominant = '0'
            for j in oxygen[i]:
                if j[i] == dominant:
                    results.append(j)
            oxygen.append(results.copy())

        co2 = [file.copy()]
        for i in range(len(file[0])):
            count0, count1, results = 0, 0, []
            for j in co2[i]:
                if j[i] == '0':
                    count0 += 1
                elif j[i] == '1':
                    count1 += 1
            if count0 <= count1:
                dominant = '0'
            else:
                dominant = '1'
            for j in co2[i]:
                if j[i] == dominant:
                    results.append(j)
            co2.append(results.copy())

        for i in range(len(oxygen)):
            if len(oxygen[i]) == 1:
                oxygen_result = int(oxygen[i][0], 2)
            if len(co2[i]) == 1:
                co2_result = int(co2[i][0], 2)

        print(oxygen_result * co2_result)
