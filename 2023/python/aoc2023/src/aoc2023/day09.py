import itertools

def pairwise(iterable):
    # pairwise('ABCDEFG') --> AB BC CD DE EF FG
    a, b = itertools.tee(iterable)
    next(b, None)
    return zip(a, b)

def run_a(input: str):
    hs = [[int(n) for n in l.rstrip().split(" ")] for l in input]
    nv = []
    for h in hs:
        delta = h
        deltas = [delta]
        while True:
            delta = [p[1]-p[0] for p in pairwise(delta)]
            deltas.append(delta)
            if not any(delta):
                break

        deltas[-1].append(0)
        for i in range(len(deltas)-2, -1, -1):
            deltas[i].append(deltas[i+1][-1] + deltas[i][-1])
        nv.append(deltas[0][-1])
    print(sum(nv))


def run_b(input: str):
    hs = [[int(n) for n in l.rstrip().split(" ")] for l in input]
    nv = []
    for h in hs:
        delta = h
        deltas = [delta]
        while True:
            delta = [p[1]-p[0] for p in pairwise(delta)]
            deltas.append(delta)
            if not any(delta):
                break

        deltas[-1].insert(0, 0)
        for i in range(len(deltas)-2, -1, -1):
            deltas[i].insert(0, deltas[i][0] - deltas[i+1][0])
        nv.append(deltas[0][0])
    print(sum(nv))
