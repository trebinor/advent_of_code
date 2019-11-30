import itertools
from z3 import *


def run_a(input: list[str]):
    sum = 0
    v = []
    TEST_MIN = 200000000000000
    TEST_MAX = 400000000000000
    # TEST_MIN = 7
    # TEST_MAX = 27
    for l in input:
        hail = l.strip().split(" @ ")
        pos = [int(p.strip()) for p in hail[0].split(",")]
        vel = [int(p.strip()) for p in hail[1].split(",")]
        v.append((pos, vel))

    smashes = 0
    for combo in itertools.combinations(v, 2):
        pos1, pos2 = (combo[0][0][0], combo[0][0][1]), (combo[1][0][0], combo[1][0][1])
        delta1, delta2 = (combo[0][1][0], combo[0][1][1]), (combo[1][1][0], combo[1][1][1])
        H1 = general_line(pos1, (pos1[0] + delta1[0], pos1[1] + delta1[1]))
        H2 = general_line(pos2, (pos2[0] + delta2[0], pos2[1] + delta2[1]))
        p = intersection(H1, H2)
        if p and TEST_MIN <= p[0] <= TEST_MAX and TEST_MIN <= p[1] <= TEST_MAX:
            if (
                (abs(p[0] - pos1[0]) > abs(p[0] - (pos1[0] + delta1[0])))
                and (abs(p[1] - pos1[1]) > abs(p[1] - (pos1[1] + delta1[1])))
                and (abs(p[0] - pos2[0]) > abs(p[0] - (pos2[0] + delta2[0])))
                and (abs(p[1] - pos2[1]) > abs(p[1] - (pos2[1] + delta2[1])))
            ):
                # print(f"Intersection inside at {p[0]} {p[1]} for {combo}")
                smashes += 1
            else:
                # print(f"Intersection inside at {p[0]} {p[1]} for {combo}, but in the past!")
                pass
        elif p:
            # print(f"Intersection outside at {p[0]} {p[1]} for {combo}")
            pass
        else:
            # print(f"No intersection for {combo} (likely div 0)")
            pass
    print(smashes)


def run_b(input: str):
    v = []
    for l in input:
        hail = l.strip().split(" @ ")
        pos = [int(p.strip()) for p in hail[0].split(",")]
        vel = [int(p.strip()) for p in hail[1].split(",")]
        v.append((pos, vel))
    s = Solver()
    z3x, z3y, z3z, z3vx, z3vy, z3vz = Real("z3x"), Real("z3y"), Real("z3z"), Real("z3vx"), Real("z3vy"), Real("z3vz")
    for pi in range(len(v)):
        ti = Real(f"t{pi}")
        pos, vel = v[pi]
        s.add(z3x + ti * z3vx == pos[0] + ti * vel[0])
        s.add(z3y + ti * z3vy == pos[1] + ti * vel[1])
        s.add(z3z + ti * z3vz == pos[2] + ti * vel[2])
    if s.check() == sat:
        m = s.model()
        x = str(m.evaluate(m.evaluate(z3x)))
        assert x.isnumeric(), "x component did not solve with an integer solution"
        y = str(m.evaluate(m.evaluate(z3y)))
        assert y.isnumeric(), "y component did not solve with an integer solution"
        z = str(m.evaluate(m.evaluate(z3z)))
        assert z.isnumeric(), "z component did not solve with an integer solution"
        print(sum(int(val) for val in [x, y, z]))

def general_line(p1, p2):
    # print(f"General line between {p1} and {p2}")
    A = p1[1] - p2[1]
    B = p2[0] - p1[0]
    C = p1[0] * p2[1] - p2[0] * p1[1]
    return (A, B, C)


def intersection(l1, l2):
    A1, B1, C1 = l1
    A2, B2, C2 = l2
    try:
        xi, yi = (
            (B1 * C2 - B2 * C1) / (A1 * B2 - A2 * B1),
            (C1 * A2 - C2 * A1) / (A1 * B2 - A2 * B1),
        )
        return (xi, yi)
    except:
        return None
