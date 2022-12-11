# correct answers:
# 1: 1833
# 2: 3425


with open('input.txt', 'r') as f:
        buffer = f.readline()

# both first and second
for amount in [4, 14]:
    for i in range(0, len(buffer) - amount):
        if len(buffer[i:i + amount]) == len(set(buffer[i: i + amount])):
            print(i + amount)
            break
