from dataclasses import dataclass
import heapq

UP = 0
RT = 1
DN = 2
LT = 3
DIRS = [UP, RT, DN, LT]

grid = []
def run_a(input: str):
    global grid
    grid = []
    for li in input:
        line = []
        for c in li.rstrip():
            line.append(int(c))
        grid.append(line)
    # print_grid()

    # using tutorial at https://www.redblobgames.com/pathfinding/a-star/introduction.html
    start_right = Node(0, 0, RT, 0)
    start_down = Node(0, 0, DN, 0)
    frontier = PriorityQueue()
    frontier.put(start_right, 0)
    frontier.put(start_down, 0)
    came_from = dict()
    cumulative_cost = dict()
    came_from[start_right] = None
    came_from[start_down] = None
    cumulative_cost[start_right] = 0
    cumulative_cost[start_down] = 0
    end = Node(get_height(), get_width(), RT, 0)

    while not frontier.empty():
        current_node = frontier.get()

        # early exit
        if current_node.row == end.row and current_node.col == end.col:
            break

        for next in get_neighbors(current_node):
            new_cost = cumulative_cost[current_node] + grid[next.row][next.col]
            if next not in cumulative_cost or new_cost < cumulative_cost[next]:
                cumulative_cost[next] = new_cost
                priority = new_cost
                frontier.put(next, priority)
                came_from[next] = current_node

    # find proper end in came_from since dir and run can vary
    current_node = None
    for k in came_from.keys():
        if k.row == end.row and k.col == end.col:
            current_node = came_from[k]
            break
    assert current_node

    path = []
    path.append(end)
    while current_node != start_right or current_node != start_down:
        path.append(current_node)
        current_node = came_from[current_node]
    path.reverse()
    print(sum([grid[p.row][p.col] for p in path]))


def run_b(input: str):
    global grid
    grid = []
    for li in input:
        line = []
        for c in li.rstrip():
            line.append(int(c))
        grid.append(line)
    # print_grid()

    start_right = Node(0, 0, RT, 0)
    start_down = Node(0, 0, DN, 0)
    frontier = PriorityQueue()
    frontier.put(start_right, 0)
    frontier.put(start_down, 0)
    came_from = dict()
    cumulative_cost = dict()
    came_from[start_right] = None
    came_from[start_down] = None
    cumulative_cost[start_right] = 0
    cumulative_cost[start_down] = 0
    end = Node(get_height(), get_width(), RT, 0)

    while not frontier.empty():
        current_node = frontier.get()

        # early exit
        if current_node.row == end.row and current_node.col == end.col:
            break

        for next in get_neighbors_b(current_node):
            new_cost = cumulative_cost[current_node] + grid[next.row][next.col]
            if next not in cumulative_cost or new_cost < cumulative_cost[next]:
                cumulative_cost[next] = new_cost
                priority = new_cost
                frontier.put(next, priority)
                came_from[next] = current_node

    # find proper end in came_from since dir and run can vary
    current_node = None
    for k in came_from.keys():
        if k.row == end.row and k.col == end.col:
            current_node = came_from[k]
            break
    assert current_node

    path = []
    path.append(end)
    while current_node != start_right or current_node != start_down:
        path.append(current_node)
        current_node = came_from[current_node]
    path.reverse()
    print(sum([grid[p.row][p.col] for p in path]))


@dataclass(frozen=True, eq=True)
class Node:
    row: int
    col: int
    dir: int
    run: int # previous blocks traveled in this direction

    def __lt__(self, other):
        return grid[self.row][self.col] < grid[other.row][other.col]

    def __eq__(self, other):
        return self.row == other.row and self.col == other.col


class PriorityQueue:
    def __init__(self):
        self.elements: list[tuple[float, Node]] = []

    def empty(self) -> bool:
        return not self.elements

    def put(self, item: Node, priority: float):
        heapq.heappush(self.elements, (priority, item))

    def get(self) -> Node:
        return heapq.heappop(self.elements)[1]


def print_grid():
    for r in grid:
        for c in r:
            print(c, end="")
        print()


def get_width():
    return len(grid[0])-1


def get_height():
    return len(grid)-1


def get_neighbors(node):
    n = []
    if node.dir == RT:
        if node.run < 3 and node.col < get_width():
            n.append(Node(node.row, node.col + 1, RT, node.run + 1))
        if node.row > 0:
            n.append(Node(node.row - 1, node.col, UP, 1))
        if node.row < get_height():
            n.append(Node(node.row + 1, node.col, DN, 1))
    elif node.dir == UP:
        if node.run < 3 and node.row > 0:
            n.append(Node(node.row - 1, node.col, UP, node.run + 1))
        if node.col > 0:
            n.append(Node(node.row, node.col - 1, LT, 1))
        if node.col < get_width():
            n.append(Node(node.row, node.col + 1, RT, 1))
    elif node.dir == DN:
        if node.run < 3 and node.row < get_height():
            n.append(Node(node.row + 1, node.col, DN, node.run + 1))
        if node.col > 0:
            n.append(Node(node.row, node.col - 1, LT, 1))
        if node.col < get_width():
            n.append(Node(node.row, node.col + 1, RT, 1))
    elif node.dir == LT:
        if node.run < 3 and node.col > 0:
            n.append(Node(node.row, node.col - 1, LT, node.run + 1))
        if node.row > 0:
            n.append(Node(node.row - 1, node.col, UP, 1))
        if node.row < get_height():
            n.append(Node(node.row + 1, node.col, DN, 1))
    else:
        assert False

    for nn in n:
        yield nn

def get_neighbors_b(node):
    n = []
    if node.dir == RT:
        if node.row == get_height() and node.col == get_width()-1:
            if node.run >= 3 and node.run < 9:
                n.append(Node(node.row, node.col + 1, RT, node.run + 1))
        elif node.run < 10 and node.col < get_width():
            n.append(Node(node.row, node.col + 1, RT, node.run + 1))
        if node.run >= 4 and node.row > 0:
            n.append(Node(node.row - 1, node.col, UP, 1))
        if node.run >= 4 and node.row < get_height() and not (node.row == get_height()-1 and node.col == get_width()):
            n.append(Node(node.row + 1, node.col, DN, 1))
    elif node.dir == UP:
        if node.run < 10 and node.row > 0:
            n.append(Node(node.row - 1, node.col, UP, node.run + 1))
        if node.run >= 4 and node.col > 0:
            n.append(Node(node.row, node.col - 1, LT, 1))
        if node.run >= 4 and node.col < get_width():
            n.append(Node(node.row, node.col + 1, RT, 1))
    elif node.dir == DN:
        if node.row == get_height()-1 and node.col == get_width():
            if node.run >= 3 and node.run < 9:
                n.append(Node(node.row + 1, node.col, DN, node.run + 1))
        elif node.run < 10 and node.row < get_height():
            n.append(Node(node.row + 1, node.col, DN, node.run + 1))
        if node.run >= 4 and node.col > 0:
            n.append(Node(node.row, node.col - 1, LT, 1))
        if node.run >= 4 and node.col < get_width() and not (node.col == get_width()-1 and node.row == get_height()):
            n.append(Node(node.row, node.col + 1, RT, 1))
    elif node.dir == LT:
        if node.run < 10 and node.col > 0:
            n.append(Node(node.row, node.col - 1, LT, node.run + 1))
        if node.run >= 4 and node.row > 0:
            n.append(Node(node.row - 1, node.col, UP, 1))
        if node.run >= 4 and node.row < get_height():
            n.append(Node(node.row + 1, node.col, DN, 1))
    else:
        assert False

    for nn in n:
        yield nn
