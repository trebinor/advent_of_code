import numpy as np

ROW = 1
COL = 2

def run_a(input: str):
    patterns = [line.rstrip().split("\n") for line in input.split("\n\n")]
    for pi, p in enumerate(patterns):
        for ri, r in enumerate(p):
            patterns[pi][ri] = [c for c in r]

    n = []
    for p in patterns:
        arr = np.array(p)
        arr.reshape(len(arr[0]), len(arr))
        n.append(np.array(p))

    reflections = []
    for p in n:
        reflections.append(find_reflection(p))
    print(score_reflections(reflections))

def run_b(input: str):
    patterns = [line.rstrip().split("\n") for line in input.split("\n\n")]
    for pi, p in enumerate(patterns):
        for ri, r in enumerate(p):
            patterns[pi][ri] = [c for c in r]

    n = []
    for p in patterns:
        arr = np.array(p)
        arr.reshape(len(arr[0]), len(arr))
        n.append(np.array(p))

    reflections = []
    for pi, p in enumerate(n):
        reflections.append(find_reflection_with_smudges(p))
    print(score_reflections(reflections))

def find_reflection(pattern):
    for xi, x in enumerate(pattern):
        if xi > len(pattern) - 1:
            continue
        span = 0
        while True:
            if xi - span < 0 or len(pattern) <= xi + span + 1:
                if span == 0:
                    break
                return (xi, span, ROW)
            if np.array_equal(pattern[xi - span], pattern[xi + span + 1]):
                span += 1
            else:
                break
    for xi, x in enumerate(pattern.T):
        if xi > len(pattern.T) - 1:
            continue
        span = 0
        while True:
            if xi - span < 0 or len(pattern.T) <= xi + span + 1:
                if span == 0:
                    break
                return (xi, span, COL)
            if np.array_equal(pattern.T[xi - span], pattern.T[xi + span + 1]):
                span += 1
            else:
                break

def find_reflection_with_smudges(pattern):
    unsmudged = find_reflection(pattern)
    prev_smudge = (0, 0)
    for ii, i in enumerate(pattern):
        for jj, j in enumerate(i):
            # unsmudge previous smudge
            if (ii, jj) != (0, 0):
                smudge(pattern, prev_smudge[0], prev_smudge[1])
                prev_smudge = (ii, jj)

            smudge(pattern, ii, jj)
            for xi, x in enumerate(pattern):
                if xi > len(pattern) - 1:
                    continue
                span = 0
                while True:
                    if xi - span < 0 or len(pattern) <= xi + span + 1:
                        if span == 0:
                            break
                        if (xi, span, ROW) != unsmudged:
                            smudge(pattern, ii, jj)
                            return (xi, span, ROW)
                        break
                    if np.array_equal(pattern[xi - span], pattern[xi + span + 1]):
                        span += 1
                    else:
                        break
            for xi, x in enumerate(pattern.T):
                if xi > len(pattern.T) - 1:
                    continue
                span = 0
                while True:
                    if xi - span < 0 or len(pattern.T) <= xi + span + 1:
                        if span == 0:
                            break
                        if (xi, span, COL) != unsmudged:
                            smudge(pattern, ii, jj)
                            return (xi, span, COL)
                        break
                    if np.array_equal(pattern.T[xi - span], pattern.T[xi + span + 1]):
                        span += 1
                    else:
                        break

def smudge(pattern, i, j):
    assert pattern[i][j] == "#" or pattern[i][j] == "."
    pattern[i][j] = "#" if pattern[i][j] == "." else "."

def score_reflections(reflections: list) -> int:
    sum = 0
    for r in reflections:
        assert r
        assert r[2] == ROW or r[2] == COL
        if r[2] == ROW:
            sum += 100 * (r[0]+1)
        else:
            sum += r[0]+1
    return sum

def print_np(pattern):
    print(np.array2string(pattern, separator="", formatter={'str_kind': lambda x: x}))

def smudge_string(s: str, i: int):
    return s[:i] + "." + s[i + 1:] if s[i] == "#" else s[:i] + "#" + s[i + 1:]

