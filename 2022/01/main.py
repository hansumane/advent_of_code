# correct answers:
# 1: 69912
# 2: 208180


with open('input.txt', 'r') as f:
    text = [list(map(int, group.strip().split('\n')))
            for group in f.read().split('\n\n')]
sums = sorted(map(sum, text), reverse=True)

# first
print(sum(sums[:1]))

# second
print(sum(sums[:3]))
