import copy
import sys
sys.setrecursionlimit(10000)

BSP = ord('\\')
FSP = ord('/')
VSP = ord('|')
HSP = ord('-')
SPC = ord('.')

UP = 0
RT = 1
DN = 2
LT = 3
DIRS = [UP, RT, DN, LT]

memo = {}
def run_a(input: str):
    grid = []
    for li in input:
        line = []
        for c in li.rstrip():
            line.append([ord(c), 0])
        grid.append(line)
    energize(grid, 0, 0, RT)
    print(sum([c[1] > 0 for li in grid for c in li]))

def run_b(input: str):
    grid = []
    for li in input:
        line = []
        for c in li.rstrip():
            line.append([ord(c), 0])
        grid.append(line)

    best = 0
    for dir in [UP, DN]:
        for i in range(0, len(grid[0])):
            grid_copy = copy.deepcopy(grid)
            memo.clear()
            energize(grid_copy, 0, i, dir)
            best = max(best, sum([c[1] > 0 for li in grid_copy for c in li]))
    for dir in [LT, RT]:
        for i in range(0, len(grid)):
            grid_copy = copy.deepcopy(grid)
            memo.clear()
            energize(grid_copy, i, 0, dir)
            best = max(best, sum([c[1] > 0 for li in grid_copy for c in li]))
    print(best)

def energize(grid, r, c, dir):
    if c > len(grid[0])-1 or r > len(grid)-1 or c < 0 or r < 0:
        return
    el, en = grid[r][c]
    if (r, c, dir, en != 0) in memo:
        return
    memo[(r, c, dir, en != 0)] = (r, c, dir, en)
    if dir == RT:
        if c > len(grid[0])-1:
            return
        grid[r][c][1] += 1
        if el == BSP:
            energize(grid, r+1, c, DN)
        elif el == FSP:
            energize(grid, r-1, c, UP)
        elif el == VSP:
            energize(grid, r-1, c, UP)
            energize(grid, r+1, c, DN)
        else:
            energize(grid, r, c+1, RT)
    elif dir == DN:
        if r > len(grid)-1:
            return
        grid[r][c][1] += 1
        if el == BSP:
            energize(grid, r, c+1, RT)
        elif el == FSP:
            energize(grid, r, c-1, LT)
        elif el == HSP:
            energize(grid, r, c-1, LT)
            energize(grid, r, c+1, RT)
        else:
            energize(grid, r+1, c, DN)
    elif dir == LT:
        if c < 0:
            return
        grid[r][c][1] += 1
        if el == BSP:
            energize(grid, r-1, c, UP)
        elif el == FSP:
            energize(grid, r+1, c, DN)
        elif el == VSP:
            energize(grid, r-1, c, UP)
            energize(grid, r+1, c, DN)
        else:
            energize(grid, r, c-1, LT)
    elif dir == UP:
        if r < 0:
            return
        grid[r][c][1] += 1
        if el == BSP:
            energize(grid, r, c-1, LT)
        elif el == FSP:
            energize(grid, r, c+1, RT)
        elif el == HSP:
            energize(grid, r, c-1, LT)
            energize(grid, r, c+1, RT)
        else:
            energize(grid, r-1, c, UP)
    else:
        assert False
