# Correct answers:
# 1: SVFDLGLWV
# 2: DCVTCVPCL


with open('input.txt', 'r') as f:
    text = f.read()
text = text.split('\n\n')

# Insane barely readable code 0_0
container = list(map(lambda x: [x[i: i + 4].strip() for i in range(0, len(x), 4)], text[0].splitlines()[:-1]))
instructions = list(map(lambda x: list(map(int, x.replace('move', '').replace('from', '').replace('to', '').split())), text[1].splitlines()))

boxes = [[] for _ in range(len(container[0]))]

for i in range(len(boxes)):
    for j in range(len(container) - 1, -1, -1):
        if container[j][i] == '':
            break
        boxes[i].append(container[j][i])

for instruction in instructions:

    mv, fr, to = instruction

    # first (uncomment 3 next lines and comment second)
    # for _ in range(mv):
    #     boxes[to - 1].append(boxes[fr - 1][-1])
    #     boxes[fr - 1].pop(-1)

    # second (uncommecnt 3 next lines and comment first)
    to_mv = boxes[fr - 1][-mv:]
    boxes[to - 1] += to_mv
    boxes[fr - 1][-mv:] = []

res = ''
for line in boxes:
    res += line[-1][1:-1]
print(res)
