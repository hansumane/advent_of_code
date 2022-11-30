from math import prod
from itertools import product


if __name__ == '__main__':

    # reading numbers from input file
    with open('input.txt', 'r') as f:
        numbers = [int(line.strip()) for line in f.readlines()]

    # first fast written part
    for combination in product(numbers, repeat=2):
        if sum(combination) == 2020:
            print('1:', prod(combination))
            break

    # second fast written part
    for combination in product(numbers, repeat=3):
        if sum(combination) == 2020:
            print('2:', prod(combination))
            break
