from itertools import combinations

def run_a(input: str):
    space = []
    for l in input:
        line = []
        for c in l.rstrip():
            line.append(c)
        space.append(line)
    (sr,sc) = len(space), len(space[0])

    # find space to expand
    rows_to_expand = []
    cols_to_expand = []
    for ri, r in enumerate(space):
        if "#" not in r:
            rows_to_expand.append(ri)
    for ci, _ in enumerate(range(sc)):
        galaxy_found = False
        for ri, _ in enumerate(space):
            if space[ri][ci] == "#":
                galaxy_found = True
                break
        if not galaxy_found:
            cols_to_expand.append(ci)

    # expand space
    expanded = 0
    for row in rows_to_expand:
        space.insert(row + expanded, ["." for i in range(sc)])
        expanded += 1
    expanded = 0
    for col in cols_to_expand:
        for ri in range(len(space)):
            space[ri].insert(col + expanded, ".")
        expanded += 1

    # find galaxy coordinates
    gals = []
    for ri, r in enumerate(space):
        for ci, _ in enumerate(r):
            if space[ri][ci] == "#":
                gals.append((ri,ci))

    # calculate taxicab distance for all combinations of galaxy coordinates
    sum = 0
    for c in combinations(gals, 2):
        sum += abs(c[0][0] - c[1][0]) + abs(c[0][1] - c[1][1])
    print(sum)

def run_b(input: str):
    space = []
    for l in input:
        line = []
        for c in l.rstrip():
            line.append(None if c == "#" else 1)
        space.append(line)

    # expand space with weighting factor
    extra_space = 1000000 - 1
    for ri, r in enumerate(space):
        if None not in r:
            for ci, c in enumerate(r):
                space[ri][ci] += extra_space
    for ci, _ in enumerate(range(len(space[0]))):
        galaxy_found = False
        for ri, _ in enumerate(space):
            if space[ri][ci] is None:
                galaxy_found = True
                break
        if not galaxy_found:
            for ri, _ in enumerate(space):
                space[ri][ci] += extra_space

    # find galaxy coordinates
    gals = []
    for ri, r in enumerate(space):
        for ci, _ in enumerate(r):
            if space[ri][ci] is None:
                gals.append((ri,ci))

    # calculate taxicab distance for all combinations of galaxy coordinates, using extra spatial expansion
    sum = 0
    for c in combinations(gals, 2):
        if c[1][0] >= c[0][0]:
            r_range = range(c[0][0], c[1][0])
            for rr in r_range:
                sum += space[rr][c[0][1]] if space[rr][c[0][1]] is not None else 1
        else:
            r_range = range(c[1][0], c[0][0])
            for rr in r_range:
                sum += space[rr][c[1][1]] if space[rr][c[1][1]] is not None else 1
        if c[1][1] >= c[0][1]:
            c_range = range(c[0][1], c[1][1])
            for cr in c_range:
                sum += space[c[0][0]][cr] if space[c[0][0]][cr] is not None else 1
        else:
            c_range = range(c[1][1], c[0][1])
            for cr in c_range:
                sum += space[c[1][0]][cr] if space[c[1][0]][cr] is not None else 1
    '''
    for ri, r in enumerate(space):
        for ci, _ in enumerate(r):
            cell = space[ri][ci]
            if cell is None:
                print(f"#", end="")
            else:
                print(f"{space[ri][ci]}", end="")
        print()
    '''
    print(sum)

