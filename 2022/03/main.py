with open('input.txt', 'r') as f:
    lines = [line.strip() for line in f.readlines()]

# first part
res = 0
for line in lines:
    for symbol in line[:len(line) // 2]:
        if symbol in line[len(line) // 2:]:
            res += 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'.index(symbol) + 1
            break
print(res)

# second part
res = 0
for group in [lines[i:i + 3] for i in range(0, len(lines), 3)]:
    for symbol in group[0]:
        if symbol in group[1] and symbol in group[2]:
            res += 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'.index(symbol) + 1
            break
print(res)
