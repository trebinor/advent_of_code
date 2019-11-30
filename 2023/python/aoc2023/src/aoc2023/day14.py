ROUND = ord("O")
CUBE = ord("#")
GND = ord(".")

def run_a(input: str):
    dish = []
    for li in input:
        line = []
        for c in li.rstrip():
            line.append(ord(c))
        dish.append(line)

    north(dish)
    print(sum([len([c for c in row if c == ROUND]) * (len(dish)-ri) for ri, row in enumerate(dish)]))

def run_b(input: str):
    dish = []
    for li in input:
        line = []
        for c in li.rstrip():
            line.append(ord(c))
        dish.append(line)

    # assume pattern starts repeating by cycle 500
    # save another 500 cycles, find frequency, use modulo to extend the repeating weights to 1 billion
    START = 500
    END = 1000
    i = 0
    w = []
    for cycle in range(0, END):
        north(dish)
        west(dish)
        south(dish)
        east(dish)
        if i >= START:
            w.append(sum([len([c for c in row if c == ROUND]) * (len(dish)-ri) for ri, row in enumerate(dish)]))
        i += 1

    repeat = 0
    for i, _ in enumerate(w):
        if w[0:i+1] == w[i+1:i+i+2]:
            break
        repeat += 1
    print(w[(1_000_000_000 - START - 1) % (repeat+1)])

def north(dish):
    for ri, row in enumerate(dish):
        for ci, c in enumerate(row):
            if c == ROUND:
                i = ri
                while i > 0 and dish[i-1][ci] not in [CUBE, ROUND]:
                    dish[i-1][ci] = ROUND
                    dish[i][ci] = GND
                    i -= 1

def west(dish):
    for ri, row in enumerate(dish):
        for ci, c in enumerate(row):
            if c == ROUND:
                i = ci
                while i > 0 and dish[ri][i-1] not in [CUBE, ROUND]:
                    dish[ri][i-1] = ROUND
                    dish[ri][i] = GND
                    i -= 1

def south(dish):
    for ri in range(len(dish)-1, -1, -1):
        for ci, c in enumerate(dish[ri]):
            if c == ROUND:
                i = ri
                while i < len(dish)-1 and dish[i+1][ci] not in [CUBE, ROUND]:
                    dish[i+1][ci] = ROUND
                    dish[i][ci] = GND
                    i += 1

def east(dish):
    for ri, row in enumerate(dish):
        for ci in range(len(row)-1, -1, -1):
            if dish[ri][ci] == ROUND:
                i = ci
                while i < len(row)-1 and dish[ri][i+1] not in [CUBE, ROUND]:
                    dish[ri][i+1] = ROUND
                    dish[ri][i] = GND
                    i += 1

def print_dish(dish):
    for row in dish:
        for c in row:
            print(chr(c), end="")
        print()
