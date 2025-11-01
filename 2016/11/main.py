#!/usr/bin/env python3

def resolve(bot_i, bots, outputs, instructions):
    if len(bots[bot_i]) == 2:
        for i in instructions:
            if "from" in i and i["from"] == bot_i:
                to1, to2 = i["to"][0], i["to"][1]
                lo, hi = min(bots[bot_i]), max(bots[bot_i])
                bots[bot_i] = []

                if lo == 17 and hi == 61:
                    print("part1:", bot_i)

                if to1[0] == "output":
                    outputs[to1[1]].append(lo)
                elif to1[0] == "bot":
                    bots[to1[1]].append(lo)
                    resolve(to1[1], bots, outputs, instructions)

                if to2[0] == "output":
                    outputs[to2[1]].append(hi)
                elif to2[0] == "bot":
                    bots[to2[1]].append(hi)
                    resolve(to2[1], bots, outputs, instructions)


def main():
    instructions = []
    with open("./input.txt", "r") as f:
        for parts in (line.strip().split() for line in f.readlines()):
            if len(parts) == 6:
                instructions.append({
                    "value": int(parts[1]),
                    "to": [("bot", int(parts[5]))],
                })
            elif len(parts) == 12:
                instructions.append({
                    "from": int(parts[1]),
                    "to": [
                        (parts[5],  int(parts[6])),
                        (parts[10], int(parts[11])),
                    ]
                })
            else:
                assert len(parts) in (6, 12)

    bots, outputs = {}, {}
    for i in instructions:
        for (kind, index) in i["to"]:
            if kind == "bot":
                bots[index] = []
            if kind == "output":
                outputs[index] = []

    for i in instructions:
        if "value" in i:
            index = i["to"][0][1]
            value = i["value"]
            bots[index].append(value)
            resolve(index, bots, outputs, instructions)

    print("part2:", outputs[0][0] * outputs[1][0] * outputs[2][0])


if __name__ == "__main__":
    main()
