import copy
import sys
import itertools
from multiprocessing import Process

map_strings = ["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"]
def batched(iterable, n):
    # batched('ABCDEFG', 3) --> ABC DEF G
    if n < 1:
        raise ValueError('n must be at least one')
    it = iter(iterable)
    while batch := tuple(itertools.islice(it, n)):
        yield batch

def run_a(input: str):
    seeds = [int(s) for s in input[0].split(": ")[1].rstrip().split(" ")]
    mappings = {m: [] for m in map_strings}
    for i, l in enumerate(input):
        for m in map_strings:
            if l.startswith(m):
                j = i+1
                while len(input) > j and input[j] != "\n":
                    nums = [int(n) for n in input[j].rsplit()]
                    mappings[m].append((nums[0], nums[1], nums[2]))
                    j += 1

    min_loc = None
    for s in seeds:
        m = copy.copy(mappings)
        loc = recurse_mappings(m, s)
        if min_loc is None or min_loc > loc:
            min_loc = loc
    print(min_loc)

def run_b(input: str):
    seed_ranges = [i for i in batched([int(s) for s in input[0].split(": ")[1].rstrip().split(" ")], 2)]
    mappings = {m: [] for m in map_strings}
    for i, l in enumerate(input):
        for m in map_strings:
            if l.startswith(m):
                j = i+1
                while len(input) > j and input[j] != "\n":
                    nums = [int(n) for n in input[j].rsplit()]
                    mappings[m].append((nums[0], nums[1], nums[2]))
                    j += 1

    p = {}
    for i, r in enumerate(seed_ranges):
        p[r[0]] = Process(target=seed_range_process, args=(r[0], r[1], mappings))
        p[r[0]].start()
    for i, r in enumerate(seed_ranges):
        p[r[0]].join()


def seed_range_process(seed_base: int, seed_inc: int, mappings: dict[list[tuple[int,int]]]) -> None:
    print(f"Started {seed_base:10} {seed_inc}")
    min_loc = None
    for j, s in enumerate(range(seed_base, seed_base+seed_inc)):
        m = copy.copy(mappings)
        loc = recurse_mappings(m, s)
        if min_loc is None or min_loc > loc:
            min_loc = loc
    print(f"min {min_loc} for {seed_base} {seed_inc}")

def recurse_mappings(mappings: dict[list[tuple[int,int]]], input: int) -> int:
    for k,m in mappings.items():
        for entry in m:
            if input >= entry[1] and input < entry[1] + entry[2]:
                output = entry[0] + (input - entry[1])
                del mappings[k]
                if len(mappings) == 0:
                    return output
                else:
                    return recurse_mappings(mappings, output)
        del mappings[k]
        if len(mappings) == 0:
            return input
        else:
            return recurse_mappings(mappings, input)
