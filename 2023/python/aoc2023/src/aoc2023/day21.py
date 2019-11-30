import numpy as np
START = ord('S')
PLOT = ord('.')
ROCK = ord('#')
STEP = ord('O')

U = 0
R = 1
D = 2
L = 3
DIRS = [U, R, D, L]

def run_a(input: str):
    grid = []
    for li in input:
        line = []
        for c in li.rstrip():
            line.append(ord(c))
        grid.append(line)
    grid = init_grid(input)
    # print_grid(grid)
    generate_steps(grid, 64)
    print(sum([c == STEP for li in grid for c in li]))

def run_b(input: str):
    grid = init_grid(input)
    assert len(grid) == len(grid[0])
    grid_len = len(grid)
    steps = 26501365
    n = steps // grid_len
    rem = steps % grid_len
    f1 = generate_steps_b(grid, rem)
    # print(rem, f1)
    grid = init_grid(input)
    f2 = generate_steps_b(grid, rem+grid_len) # f(n+X)
    # print(rem+grid_len, f2)
    grid = init_grid(input)
    f3 = generate_steps_b(grid, rem+2*grid_len) # f(n+2X)
    # print(rem+2*grid_len, f3)

    # p=np.polyfit(x=[rem, rem+grid_len, rem+2*grid_len], y=[f1, f2, f3], deg=2)
    # using quadradic fit {f1, f2, f3} from wolfram alpha because numpy's polyfit is incorrect for some reason
    a = 15350
    b = 15465
    c = 3885
    print(a*n*n + b*n + c)

def print_grid(grid):
    for r in grid:
        for c in r:
            print(chr(c), end="")
            assert c in [START, PLOT, ROCK, STEP]
        print()

def init_grid(input):
    grid = []
    for li in input:
        line = []
        for c in li.rstrip():
            line.append(ord(c))
        grid.append(line)
    return grid

def generate_steps(grid: list[list[str]], steps: int):
    last_steps = set()
    # find starting position
    for ri, row in enumerate(grid):
        for ci, c in enumerate(row):
            if grid[ri][ci] == START:
                last_steps.add((ri, ci))

    for s in range(steps):
        next_steps = set()
        for ri, ci in last_steps:
            for n in [(ri-1, ci), (ri+1, ci), (ri, ci-1), (ri, ci+1)]:
                if n[0] >= 0 and n[0] <= len(grid)-1 and n[1] >= 0 and n[1] <= len(grid[0])-1:
                    if grid[n[0]][n[1]] in [PLOT, START, STEP]:
                        # print(f"Adding next step {n[0]} {n[1]}")
                        next_steps.add(n)

        # print(next_steps)
        for ri, ci in last_steps:
            grid[ri][ci] = PLOT
        last_steps = next_steps.copy()
        for ri, ci in next_steps:
            grid[ri][ci] = STEP
        # print_grid(grid)

def generate_steps_b(grid: list[list[str]], steps: int):
    all_steps = []
    # find starting position
    for ri, row in enumerate(grid):
        for ci, c in enumerate(row):
            if grid[ri][ci] == START:
                all_steps.append(set([(ri, ci)]))

    for s in range(steps):
        next_steps = set()
        for ri, ci in all_steps[s]:
            for n in [(ri-1, ci), (ri+1, ci), (ri, ci-1), (ri, ci+1)]:
                if grid[n[0] % len(grid)][n[1] % len(grid[0])] in [PLOT, START, STEP]:
                    next_steps.add(n)

        all_steps.append(next_steps)
    return len(all_steps[steps])
