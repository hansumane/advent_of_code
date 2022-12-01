with open('input.txt', 'r') as f:
    text = [list(map(int, line.strip().split('\n')))
            for line in f.read().split('\n\n')]
sums = sorted(map(sum, text), reverse=True)

# first
print(sum(sums[:1]))

# second
print(sum(sums[:3]))
