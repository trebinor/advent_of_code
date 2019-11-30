from termcolor import colored, cprint

START = ord("S")
VERT = ord("|")  # 186
HORZ = ord("-")  # 205
NE = ord("L")  # 200
NW = ord("J")  # 188
SW = ord("7")  # 187
SE = ord("F")  # 201
GND = ord(".")

UP = 0
RT = 1
DN = 2
LT = 3
DIRS = [UP, RT, DN, LT]


def run_a(input: str):
    maze = []
    for l in input:
        line = []
        for c in l.rstrip():
            line.append(ord(c))
        maze.append(line)

    # find starting position
    start = None
    break_out = False
    for ri, r in enumerate(maze):
        if break_out:
            break
        for ci, c in enumerate(r):
            if maze[ri][ci] == START:
                start = (ri, ci)
                break_out = True
                break

    steps = 0
    (r, c) = start
    last_direction = UP
    while True:
        if maze[r][c] == START and steps != 0:
            break
        # else:
        #    print(f"{r+1} {c+1} {steps}")
        for direction in rotator(last_direction):
            if (
                direction == UP
                and last_direction != DN
                and r - 1 >= 0
                and maze[r - 1][c] in [SE, SW, VERT, START]
                and maze[r][c] not in [SE, SW, HORZ]
            ):
                steps += 1
                r -= 1
                last_direction = UP
                break
            if (
                direction == RT
                and last_direction != LT
                and c + 1 < len(maze)
                and maze[r][c + 1] in [NW, SW, HORZ, START]
                and maze[r][c] not in [NW, SW, VERT]
            ):
                steps += 1
                c += 1
                last_direction = RT
                break
            if (
                direction == DN
                and last_direction != UP
                and r + 1 < len(maze[0])
                and maze[r + 1][c] in [NE, NW, VERT, START]
                and maze[r][c] not in [NE, NW, HORZ]
            ):
                steps += 1
                r += 1
                last_direction = DN
                break
            if (
                direction == LT
                and last_direction != RT
                and c - 1 >= 0
                and maze[r][c - 1] in [NE, SE, HORZ, START]
                and maze[r][c] not in [NE, SE, VERT]
            ):
                steps += 1
                c -= 1
                last_direction = LT
                break

    # print_maze(maze, marked_maze)
    # farthest position is the halfway point for either direction around the loop
    print(steps // 2)


def run_b(input: str):
    maze = []
    for l in input:
        line = []
        for c in l.rstrip():
            line.append(ord(c))
        maze.append(line)
    marked_maze = [["?" for c in r] for r in maze]

    # find starting position
    start = None
    break_out = False
    for ri, r in enumerate(maze):
        if break_out:
            break
        for ci, c in enumerate(r):
            if maze[ri][ci] == START:
                start = (ri, ci)
                break_out = True
                break

    steps = 0
    (r, c) = start
    print(start)
    last_direction = UP
    while True:
        if maze[r][c] == START and steps != 0:
            break
        for direction in rotator(last_direction):
            if (
                direction == UP
                and last_direction != DN
                and r - 1 >= 0
                and maze[r - 1][c] in [SE, SW, VERT, START]
                and maze[r][c] not in [SE, SW, HORZ]
            ):
                steps += 1
                r -= 1
                last_direction = UP
                marked_maze[r][c] = "M"
                break
            if (
                direction == RT
                and last_direction != LT
                and c + 1 < len(maze)
                and maze[r][c + 1] in [NW, SW, HORZ, START]
                and maze[r][c] not in [NW, SW, VERT]
            ):
                steps += 1
                c += 1
                last_direction = RT
                marked_maze[r][c] = "M"
                break
            if (
                direction == DN
                and last_direction != UP
                and r + 1 < len(maze[0])
                and maze[r + 1][c] in [NE, NW, VERT, START]
                and maze[r][c] not in [NE, NW, HORZ]
            ):
                steps += 1
                r += 1
                last_direction = DN
                marked_maze[r][c] = "M"
                break
            if (
                direction == LT
                and last_direction != RT
                and c - 1 >= 0
                and maze[r][c - 1] in [NE, SE, HORZ, START]
                and maze[r][c] not in [NE, SE, VERT]
            ):
                steps += 1
                c -= 1
                last_direction = LT
                marked_maze[r][c] = "M"
                break

    inside = [
        (36, 38),
        (36, 64),
        (36, 89),
        (38, 47),
        (38, 88),
        (39, 58),
        (39, 73),
        (40, 39),
        (40, 50),
        (40, 83),
        (40, 94),
        (41, 68),
        (41, 76),
        (41, 88),
        (41, 91),
        (42, 62),
        (42, 73),
        (42, 80),
        (42, 97),
        (43, 39),
        (43, 58),
        (43, 85),
        (43, 96),
        (44, 51),
        (44, 78),
        (44, 102),
        (45, 44),
        (45, 100),
        (47, 38),
        (47, 91),
        (48, 56),
        (48, 63),
        (48, 66),
        (49, 43),
        (49, 71),
        (49, 84),
        (49, 101),
        (50, 40),
        (51, 92),
        (52, 88),
        (53, 42),
        (53, 58),
        (54, 48),
        (54, 63),
        (54, 79),
        (55, 63),
        (57, 39),
        (57, 42),
        (57, 79),
        (58, 40),
        (58, 43),
        (58, 54),
        (59, 61),
        (59, 62),
        (60, 62),
        (60, 63),
        (60, 64),
        (60, 69),
        (60, 76),
        (60, 99),
        (61, 62),
        (61, 63),
        (61, 64),
        (61, 73),
        (61, 88),
        (62, 36),
        (62, 63),
        (62, 64),
        (62, 65),
        (62, 66),
        (62, 67),
        (62, 68),
        (62, 73),
        (63, 48),
        (63, 63),
        (63, 64),
        (63, 65),
        (63, 66),
        (63, 67),
        (63, 68),
        (63, 69),
        (63, 70),
        (63, 73),
        (63, 74),
        (63, 75),
        (64, 39),
        (64, 58),
        (64, 63),
        (64, 64),
        (64, 65),
        (64, 66),
        (64, 67),
        (64, 68),
        (64, 69),
        (64, 70),
        (64, 71),
        (64, 72),
        (64, 73),
        (64, 74),
        (64, 75),
        (65, 65),
        (65, 66),
        (65, 67),
        (65, 68),
        (65, 69),
        (65, 70),
        (65, 71),
        (65, 72),
        (65, 73),
        (65, 74),
        (65, 75),
        (65, 76),
        (66, 65),
        (66, 66),
        (66, 67),
        (66, 68),
        (66, 69),
        (66, 70),
        (66, 71),
        (66, 72),
        (66, 73),
        (66, 74),
        (66, 75),
        (66, 76),
        (66, 79),
        (66, 80),
        (66, 86),
        (67, 65),
        (67, 66),
        (67, 67),
        (67, 68),
        (67, 69),
        (67, 70),
        (67, 71),
        (67, 72),
        (67, 73),
        (67, 74),
        (67, 75),
        (67, 76),
        (67, 77),
        (67, 78),
        (67, 79),
        (67, 80),
        (68, 65),
        (68, 66),
        (68, 67),
        (68, 68),
        (68, 69),
        (68, 70),
        (68, 71),
        (68, 72),
        (68, 73),
        (68, 74),
        (68, 75),
        (68, 76),
        (68, 77),
        (68, 78),
        (68, 79),
        (68, 80),
        (68, 81),
        (68, 101),
        (68, 104),
        (69, 65),
        (69, 66),
        (69, 67),
        (69, 68),
        (69, 69),
        (69, 70),
        (69, 71),
        (69, 72),
        (69, 73),
        (69, 74),
        (69, 75),
        (69, 76),
        (69, 77),
        (69, 78),
        (69, 95),
        (70, 40),
        (70, 53),
        (70, 65),
        (70, 66),
        (70, 67),
        (70, 68),
        (70, 69),
        (70, 70),
        (70, 71),
        (70, 72),
        (70, 73),
        (70, 74),
        (70, 75),
        (70, 76),
        (70, 77),
        (70, 78),
        (71, 59),
        (71, 60),
        (71, 63),
        (71, 64),
        (71, 65),
        (71, 66),
        (71, 67),
        (71, 68),
        (71, 69),
        (71, 70),
        (71, 71),
        (71, 72),
        (71, 73),
        (71, 74),
        (71, 75),
        (72, 62),
        (72, 63),
        (72, 64),
        (72, 65),
        (72, 66),
        (72, 67),
        (72, 68),
        (72, 69),
        (72, 70),
        (72, 71),
        (72, 72),
        (72, 73),
        (72, 74),
        (72, 75),
        (73, 54),
        (73, 62),
        (73, 63),
        (73, 64),
        (73, 65),
        (73, 66),
        (73, 67),
        (73, 68),
        (73, 69),
        (73, 70),
        (73, 71),
        (73, 72),
        (73, 73),
        (73, 74),
        (73, 90),
        (74, 61),
        (74, 62),
        (74, 63),
        (74, 64),
        (74, 65),
        (74, 66),
        (74, 67),
        (74, 68),
        (74, 69),
        (74, 70),
        (74, 71),
        (74, 72),
        (74, 73),
        (74, 74),
        (75, 40),
        (75, 51),
        (75, 61),
        (75, 62),
        (75, 63),
        (75, 64),
        (75, 65),
        (75, 66),
        (75, 67),
        (75, 68),
        (75, 69),
        (75, 70),
        (75, 71),
        (75, 72),
        (75, 73),
        (75, 74),
        (75, 75),
        (75, 76),
        (75, 77),
        (75, 99),
        (76, 57),
        (76, 62),
        (76, 63),
        (76, 64),
        (76, 65),
        (76, 66),
        (76, 67),
        (76, 68),
        (76, 69),
        (76, 70),
        (76, 71),
        (76, 72),
        (76, 73),
        (76, 74),
        (76, 75),
        (76, 78),
        (76, 79),
        (76, 82),
        (76, 83),
        (76, 86),
        (77, 62),
        (77, 63),
        (77, 64),
        (77, 67),
        (77, 68),
        (77, 69),
        (77, 70),
        (77, 71),
        (77, 72),
        (77, 73),
        (77, 74),
        (77, 75),
        (78, 67),
        (78, 68),
        (78, 69),
        (78, 70),
        (78, 71),
        (78, 72),
        (78, 73),
        (78, 74),
        (78, 75),
        (78, 93),
        (79, 43),
        (79, 70),
        (79, 71),
        (79, 72),
        (79, 73),
        (79, 74),
        (79, 75),
        (79, 76),
        (79, 77),
        (79, 95),
        (80, 47),
        (80, 64),
        (80, 70),
        (80, 71),
        (80, 72),
        (80, 73),
        (80, 74),
        (80, 75),
        (80, 76),
        (80, 77),
        (80, 97),
        (81, 40),
        (81, 51),
        (81, 62),
        (81, 63),
        (81, 67),
        (81, 68),
        (81, 69),
        (81, 73),
        (81, 74),
        (81, 75),
        (81, 76),
        (81, 77),
        (82, 50),
        (82, 63),
        (82, 68),
        (82, 69),
        (82, 75),
        (82, 76),
        (82, 77),
        (83, 41),
        (83, 70),
        (84, 67),
        (84, 71),
        (84, 92),
        (85, 83),
        (85, 98),
        (86, 38),
        (86, 74),
        (87, 71),
        (88, 70),
        (88, 71),
        (89, 52),
        (89, 80),
        (91, 36),
        (91, 63),
        (91, 70),
        (91, 71),
        (91, 99),
        (92, 84),
        (93, 83),
        (93, 88),
        (93, 104),
        (94, 44),
        (94, 60),
        (94, 67),
        (94, 87),
        (94, 99),
        (95, 74),
        (96, 65),
        (96, 90),
        (96, 91),
        (96, 94),
        (97, 46),
        (97, 53),
        (97, 69),
        (98, 44),
        (98, 47),
        (98, 70),
        (98, 87),
        (99, 82),
        (101, 44),
        (101, 79),
        (102, 102),
        (104, 36),
        (104, 59),
        (104, 81),
        (104, 82),
    ]
    for cell in inside:
        assert(marked_maze[cell[0]][cell[1]] != "M")
        marked_maze[cell[0]][cell[1]] = "I"
    print_maze(maze, marked_maze)
    print(len(inside))

    # I solved the problem by counting the inside tiles in the visualization above. Someone else pointed out you can
    # calculate this by using a bounding rectangle (35 to 105) x (35 to 105) and subtracting the loop tiles. This works,
    # but it's only obvious in retrospect that a bounding rectangle does that as there might be non-obvious inner tiles
    # outside the loop.  Fill flood would have been a more elegant solution.
    inside = 0
    bounds = ((35, 105), (35, 105))
    for i in range(bounds[0][0], bounds[0][1]):
        for j in range(bounds[1][0], bounds[1][1]):
            if marked_maze[i][j] == "M":
                inside += 1
    print((bounds[0][1] - bounds[0][0]) * (bounds[1][1] - bounds[1][0]) - inside)


def rotator(starting_direction: int):
    for i in range(starting_direction, starting_direction + len(DIRS)):
        yield i % len(DIRS)


def print_maze(maze, marked_maze):
    def extended(code: int) -> str:
        return bytes([code]).decode("cp437")

    def print_cell(maze, marked_maze, r, c) -> None:
        if marked_maze[r][c] == "M":
            if maze[r][c] == VERT:
                print(colored(extended(186), "red", "on_green"), end="")
            elif maze[r][c] == HORZ:
                print(colored(extended(205), "red", "on_green"), end="")
            elif maze[r][c] == NE:
                print(colored(extended(200), "red", "on_green"), end="")
            elif maze[r][c] == NW:
                print(colored(extended(188), "red", "on_green"), end="")
            elif maze[r][c] == SW:
                print(colored(extended(187), "red", "on_green"), end="")
            elif maze[r][c] == SE:
                print(colored(extended(201), "red", "on_green"), end="")
            elif maze[r][c] == START:
                print(colored(chr(START), "green", "on_red"), end="")
        elif marked_maze[r][c] == "I":
            if maze[r][c] == VERT:
                print(colored(extended(186), "green", "on_blue"), end="")
            elif maze[r][c] == HORZ:
                print(colored(extended(205), "green", "on_blue"), end="")
            elif maze[r][c] == NE:
                print(colored(extended(200), "green", "on_blue"), end="")
            elif maze[r][c] == NW:
                print(colored(extended(188), "green", "on_blue"), end="")
            elif maze[r][c] == SW:
                print(colored(extended(187), "green", "on_blue"), end="")
            elif maze[r][c] == SE:
                print(colored(extended(201), "green", "on_blue"), end="")
            elif maze[r][c] == GND:
                print(colored(chr(GND), "green", "on_blue"), end="")
            else:
                print(colored("I", "green"), end="")
        else:
            if maze[r][c] == VERT:
                print(colored(extended(186), "white"), end="")
            elif maze[r][c] == HORZ:
                print(colored(extended(205), "white"), end="")
            elif maze[r][c] == NE:
                print(colored(extended(200), "white"), end="")
            elif maze[r][c] == NW:
                print(colored(extended(188), "white"), end="")
            elif maze[r][c] == SW:
                print(colored(extended(187), "white"), end="")
            elif maze[r][c] == SE:
                print(colored(extended(201), "white"), end="")
            elif maze[r][c] == GND:
                print(colored(chr(GND), "white"), end="")
            elif maze[r][c] == START:
                print(colored(chr(START), "green"), end="")

    print("    ", end="")
    for ci, _ in enumerate(maze[0]):
        if ci < 100:
            print("0", end="")
        else:
            print("1", end="")
    print()
    print("    ", end="")
    for ci, _ in enumerate(maze[0]):
        print(f"{(ci // 10) % 10}", end="")
    print()
    print("    ", end="")
    for ci, _ in enumerate(maze[0]):
        print(f"{ci % 10}", end="")
    print()
    for ri, r in enumerate(maze):
        print(f"{ri:03} ", end="")
        for ci, c in enumerate(r):
            print_cell(maze, marked_maze, ri, ci)
        print()
