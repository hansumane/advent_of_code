# Correct answers:
# 1: 12980
# 2: BRJLFULP


def add_sgn():
    global sgn
    if cc == 20 or cc >= 60 and (cc - 20) % 40 == 0:
        sgn += cc * x

def draw_pixel():
    global pixels
    sprite = '.' * (x - 1) + '###'
    try:
        pixels += sprite[(cc - 1) % 40]
    except:
        pixels += '.'

with open('input.txt', 'r') as f:
    lines = [line.strip().split() for line in f.readlines()]
    lines = list(map(lambda x: [x[0], int(x[1])] if len(x) > 1 else [x[0], None], lines))

x, cc, sgn, pixels = 1, 1, 0, ''

for line in lines:
    cmd, value = line
    add_sgn(); draw_pixel()
    if cmd == 'noop':
        cc += 1
    elif cmd == 'addx':
        cc += 1
        add_sgn(); draw_pixel()
        cc += 1
        x += value

print(sgn, *(pixels[i:i + 40] for i in range(0, len(pixels), 40)), sep='\n')
