# correct answers:
# 1: 515
# 2: 883

with open('input.txt', 'r') as f:
    lines = [line.strip().split(',') for line in f.readlines()]
    lines = [list(map(lambda x: list(map(int, x.split('-'))), line)) for line in lines]

# first
res = 0
for line in lines:
    if ((line[0][0] >= line[1][0] and line[0][1] <= line[1][1])
            or (line[1][0] >= line[0][0] and line[1][1] <= line[0][1])):
        res += 1
print(res)

# second
res = 0
for line in lines:
    if ((line[0][0] <= line[1][0] <= line[0][1])
            or (line[0][0] <= line[1][1] <= line[0][1])
            or (line[1][0] <= line[0][0] <= line[1][1])
            or (line[1][0] <= line[0][1] <= line[1][1])):
        res += 1
print(res)
