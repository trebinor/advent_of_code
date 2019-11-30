from dataclasses import dataclass

PATH = ord(".")
TREE = ord("#")
RSL = ord(">")
DSL = ord("v")


def run_a(input: str):
    grid = []
    for li in input:
        line = []
        for c in li.rstrip():
            line.append(ord(c))
        grid.append(line)
    # print_grid(grid)

    visited = set()
    start = (0, 1)
    end = (get_height(grid), get_width(grid) - 1)
    path_points = [start, end]
    network = {point: {} for point in path_points}
    for ri, r in enumerate(grid):
        for ci, c in enumerate(r):
            if c in [TREE]:
                continue
            neighbor_count = 0
            for nr, nc in [(ri - 1, ci), (ri + 1, ci), (ri, ci - 1), (ri, ci + 1)]:
                if nr >= 0 and nr <= get_height(grid) and nc >= 0 and nc <= get_width(grid) and grid[nr][nc] not in [TREE]:
                    neighbor_count += 1
            if neighbor_count > 2:
                path_points.append((ri, ci))

    network = {point: {} for point in path_points}
    for point in path_points:
        visited = set()
        visited.add(point)
        explore = [(point[0], point[1], 0)]
        while len(explore) > 0:
            r, c, n = explore.pop()
            if n != 0 and (r, c) in path_points:
                network[point][(r, c)] = n
                continue
            neighbors = [nb for nb in get_neighbors(grid, Node(r, c))]
            for node in neighbors:
                if (
                    (node.row, node.col) not in visited
                    and node.row >= 0
                    and node.row <= get_height(grid)
                    and node.col >= 0
                    and node.col <= get_width(grid)
                    and grid[node.row][node.col] not in [TREE]
                ):
                    visited.add((node.row, node.col))
                    explore.append((node.row, node.col, n + 1))

    visited = set()
    print(explore_with_dfs(network, visited, start, end))


def run_b(input: str):
    grid = []
    for li in input:
        line = []
        for c in li.rstrip():
            line.append(PATH if ord(c) in [RSL, DSL] else ord(c))
        grid.append(line)
    # print_grid(grid)

    start = (0, 1)
    end = (get_height(grid), get_width(grid) - 1)
    prune_points = [start, end]
    for ri, r in enumerate(grid):
        for ci, c in enumerate(r):
            if c in [TREE]:
                continue
            neighbors = [nb for nb in get_neighbors_b(grid, Node(ri, ci))]
            if len(neighbors) > 2:
                prune_points.append((ri, ci))

    network = {point: {} for point in prune_points}
    for point in prune_points:
        visited = set()
        visited.add(point)
        explore = [(point[0], point[1], 0)]
        while len(explore) > 0:
            r, c, n = explore.pop()
            if n != 0 and (r, c) in prune_points:
                network[point][(r, c)] = n
                continue
            for dir_r, dir_c in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                nr = r + dir_r
                nc = c + dir_c
                if (
                    (nr, nc) not in visited
                    and nr >= 0
                    and nr <= get_height(grid)
                    and nc >= 0
                    and nc <= get_width(grid)
                    and grid[nr][nc] not in [TREE]
                ):
                    visited.add((nr, nc))
                    explore.append((nr, nc, n + 1))

    visited = set()
    print(explore_with_dfs(network, visited, start, end))


@dataclass
class Node:
    row: int
    col: int


def print_grid(grid):
    for r in grid:
        for c in r:
            print(chr(c), end="")
            assert c in [PATH, TREE, RSL, DSL]
        print()


def get_width(grid):
    return len(grid[0]) - 1


def get_height(grid):
    return len(grid) - 1


def explore_with_dfs(network, visited, point, end):
    if point == end:
        return 0
    distance = -100000
    visited.add(point)
    for explore_point in network[point]:
        if explore_point not in visited:
            distance = max(distance, explore_with_dfs(network, visited, explore_point, end) + network[point][explore_point])
    visited.remove(point)
    return distance


def get_neighbors(grid, node):
    n = []
    if grid[node.row][node.col] == RSL:
        n.append(Node(node.row, node.col + 1))
    elif grid[node.row][node.col] == DSL:
        n.append(Node(node.row + 1, node.col))
    else:
        for d in [(node.row - 1, node.col), (node.row + 1, node.col), (node.row, node.col - 1), (node.row, node.col + 1)]:
            if d[0] >= 0 and d[0] <= get_height(grid) and d[1] >= 0 and d[1] <= get_width(grid):
                if grid[d[0]][d[1]] not in [TREE]:
                    n.append(Node(d[0], d[1]))
    for nn in n:
        yield nn


def get_neighbors_b(grid, node):
    n = []
    for d in [(node.row - 1, node.col), (node.row + 1, node.col), (node.row, node.col - 1), (node.row, node.col + 1)]:
        if d[0] >= 0 and d[0] <= get_height(grid) and d[1] >= 0 and d[1] <= get_width(grid) and grid[d[0]][d[1]] not in [TREE]:
            n.append(Node(d[0], d[1]))
    for nn in n:
        yield nn
