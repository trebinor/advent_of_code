import sys
import numpy as np
from dataclasses import dataclass
from skspatial.measurement import area_signed

def run_a(input: str):
    specs = []
    for li in input:
        fields = li.rstrip().split(" ")
        specs.append(Spec(fields[0], int(fields[1]), fields[2].strip("()")))

    size = 660
    cursor = [size // 2, size // 2]
    lagoon = np.array([np.array([0 for i in range(size)]) for j in range(size)])
    lagoon[cursor[0]][cursor[1]] = 1
    dirs = []
    for s in specs:
        assert s.dir == "R" or s.dir == "D" or s.dir == "L" or s.dir == "U"
        dirs.append(s.dir)
        if s.dir == "L":
            for i in range(1, s.dist+1):
                lagoon[cursor[1]][cursor[0]-i] = 1
            cursor[0] -= s.dist
        elif s.dir == "R":
            for i in range(1, s.dist+1):
                lagoon[cursor[1]][cursor[0]+i] = 1
            cursor[0] += s.dist
        elif s.dir == "U":
            for i in range(1, s.dist+1):
                lagoon[cursor[1]-i][cursor[0]] = 1
            cursor[1] -= s.dist
        elif s.dir == "D":
            for i in range(1, s.dist+1):
                lagoon[cursor[1]+i][cursor[0]] = 1
            cursor[1] += s.dist

    if dirs[0] in ["U", "D"] and dirs[1] in ["L"]:
        fill_cursor = ((size // 2) - 1, size // 2)
    elif dirs[0] in ["U", "D"] and dirs[1] in ["R"]:
        fill_cursor = ((size // 2) + 1, size // 2)
    elif dirs[0] in ["R"] and dirs[1] in ["U"]:
        fill_cursor = (size // 2, (size // 2) - 1)
    elif dirs[0] in ["L"] and dirs[1] in ["U"]:
        fill_cursor = ((size // 2) - 1, (size // 2) - 1)
    elif dirs[0] in ["L"] and dirs[1] in ["D"]:
        fill_cursor = ((size // 2) - 1, (size // 2) + 1)
    elif dirs[0] in ["R"] and dirs[1] in ["D"]:
        fill_cursor = ((size // 2) + 1, (size // 2) + 1)
    np.set_printoptions(threshold=sys.maxsize, linewidth=np.inf)

    flood_fill(lagoon, fill_cursor[0], fill_cursor[1])
    convert_flood_fill_to_lava(lagoon)
    print(np.sum(lagoon))


def run_b(input: str):
    specs = []
    for li in input:
        fields = li.rstrip().split(" ")
        _, _, packed_color = fields[0], int(fields[1]), fields[2].strip("()")
        specs.append(Spec(dirnum_to_dir(int(packed_color[-1])), int(packed_color[1:-1], 16), 0))

    cursor = [0, 0]
    perimeter = 0
    points = []
    points.append(cursor)
    for s in specs:
        assert s.dir == "R" or s.dir == "D" or s.dir == "L" or s.dir == "U"
        perimeter += s.dist
        if s.dir == "L":
            cursor[1] -= s.dist
        elif s.dir == "R":
            cursor[1] += s.dist
        elif s.dir == "U":
            cursor[0] -= s.dist
        elif s.dir == "D":
            cursor[0] += s.dist
        else:
            assert False
        points.append([cursor[0], cursor[1]])

    # Combine area from the shoelace formula, a perimeter calculation tracking the digger progress, and Pick's theorem
    print(int(abs(area_signed(points))) + perimeter // 2 + 1)


@dataclass
class Spec:
    dir: str
    dist: int
    color: str


def flood_fill(lagoon, row, col):
    q = {}
    q[(row, col)] = 1
    while len(q) > 0:
        (fr, fc) = list(q)[0]
        lagoon[fr][fc] = -1
        del q[(fr, fc)]
        if lagoon[fr+1][fc] == 0:
            q[(fr+1, fc)] = 1
        if lagoon[fr-1][fc] == 0:
            q[(fr-1, fc)] = 1
        if lagoon[fr][fc-1] == 0:
            q[(fr, fc-1)] = 1
        if lagoon[fr][fc+1] == 0:
            q[(fr, fc+1)] = 1

def convert_flood_fill_to_lava(lagoon):
    for ri, r in enumerate(lagoon):
        for ci, c in enumerate(r):
            if c == -1:
                lagoon[ri][ci] = 1

def dirnum_to_dir(num: int):
    if num == 0:
        return "R"
    if num == 1:
        return "D"
    if num == 2:
        return "L"
    if num == 3:
        return "U"
    assert False
