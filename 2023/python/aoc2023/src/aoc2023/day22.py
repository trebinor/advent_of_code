import copy
from functools import cmp_to_key

def run_a(input: str):
    floating_bricks = []
    for line in input:
        end1, end2 = line.strip().split("~")
        floating_bricks.append(([int(b) for b in end1.split(",")], [int(b) for b in end2.split(",")]))

    # let bricks fall sorted by ascending minimum z values so we don't have to do multiple passes
    bricks = sorted(floating_bricks, key=cmp_to_key(lambda a, b: min(a[0][2], a[1][2]) - min(b[0][2], b[1][2])))
    let_bricks_fall(bricks)

    sum = 0
    for b in bricks:
        safe = True
        for o in bricks:
            if o == b:
                continue
            if brick_supports_other(b, o) and not only_brick_supports(bricks, b, o):
                print(f"Brick {b} not safe because it supports {o} and nothing else supports that")
                safe = False
                break
        if safe:
            print(f"Brick {b} safe")
            sum += 1
    print(sum)


def run_b(input: str):
    floating_bricks = []
    for line in input:
        end1, end2 = line.strip().split("~")
        floating_bricks.append(([int(b) for b in end1.split(",")], [int(b) for b in end2.split(",")]))

    bricks = sorted(floating_bricks, key=cmp_to_key(lambda a, b: min(a[0][2], a[1][2]) - min(b[0][2], b[1][2])))
    let_bricks_fall(bricks)

    sum = 0
    for bi in range(len(bricks)):
        iteration_bricks = copy.deepcopy(bricks)
        iteration_bricks.pop(bi)
        print(f"Removed brick {bi} {bricks[bi]} new len {len(iteration_bricks)}")
        sum += count_fallen_bricks(iteration_bricks)
    print(sum)


def let_bricks_fall(bricks):
    for bi, b in enumerate(bricks):
        while not any_brick_supports(bricks, b) and b[0][2] > 1 and b[1][2] > 1:
            b[0][2], b[1][2] = b[0][2] - 1, b[1][2] - 1


def count_fallen_bricks(bricks):
    fallen_count = 0
    for bi, b in enumerate(bricks):
        fallen = False
        while not any_brick_supports(bricks, b) and b[0][2] > 1 and b[1][2] > 1:
            if not fallen:
                fallen = True
                fallen_count += 1
            b[0][2], b[1][2] = b[0][2] - 1, b[1][2] - 1
    return fallen_count


def get_vertical_adj(b):
    return (range(b[0][0], b[1][0] + 1), range(b[0][1], b[1][1] + 1), range(max(b[0][2], b[1][2])+1, max(b[0][2], b[1][2])+2))


def brick_supports_other(b, o):
    vert_adjacent = get_vertical_adj(b)
    return any([x in vert_adjacent[0] for x in range(o[0][0], o[1][0]+1)]) and any([y in vert_adjacent[1] for y in range(o[0][1], o[1][1]+1)]) and min(o[0][2], o[1][2]) in vert_adjacent[2]


def any_brick_supports(bricks, b):
    supports = False
    for o in bricks:
        if b != o and brick_supports_other(o, b):
            supports = True
            break
    return supports


def only_brick_supports(bricks, b, o):
    supports = False
    for brick in bricks:
        if brick != o and brick != b and brick_supports_other(brick, o):
            supports = True
            break
    return supports
