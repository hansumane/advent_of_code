# correct answers:
# 1: 11841
# 2: 13022

# A, X = Rock      +1
# B, Y = Paper     +2
# C, Z = Scissors  +3

#        loss      +0
#        draw      +3
#        win       +6


def first(combs):

    score = 0

    for comb in combs:

        match comb[1]:

            case 'X':
                score += 1
                match comb[0]:
                    case 'A':
                        score += 3
                    case 'B':
                        score += 0
                    case 'C':
                        score += 6

            case 'Y':
                score += 2
                match comb[0]:
                    case 'A':
                        score += 6
                    case 'B':
                        score += 3
                    case 'C':
                        score += 0

            case 'Z':
                score += 3
                match comb[0]:
                    case 'A':
                        score += 0
                    case 'B':
                        score += 6
                    case 'C':
                        score += 3

    return score


def second(combs):

    score = 0

    for comb in combs:

        match comb[1]:

            case 'X':
                score += 0
                match comb[0]:
                    case 'A':
                        score += 3
                    case 'B':
                        score += 1
                    case 'C':
                        score += 2

            case 'Y':
                score += 3
                match comb[0]:
                    case 'A':
                        score += 1
                    case 'B':
                        score += 2
                    case 'C':
                        score += 3

            case 'Z':
                score += 6
                match comb[0]:
                    case 'A':
                        score += 2
                    case 'B':
                        score += 3
                    case 'C':
                        score += 1

    return score


with open('input.txt', 'r') as f:
    lines = [line.strip().split() for line in f.readlines()]

print(first(lines))
print(second(lines))
