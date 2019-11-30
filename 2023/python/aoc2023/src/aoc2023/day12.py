import itertools
import regex


def run_a(input: str):
    sum = 0
    for li, line in enumerate(input):
        # print(li)
        gears, dmg_report = line.rstrip().split(" ")
        gears = [g for g in gears]
        dmg_list = [int(d) for d in dmg_report.split(",")]
        qlocs = [qi for qi, q in enumerate(gears) if q == "?"]
        possibilities = []
        for p in itertools.product("#.", repeat=len(qlocs)):
            gears_copy = gears
            for ci, c in enumerate(p):
                gears_copy[qlocs[ci]] = c
            if evaluate_possibility_a(gears_copy, dmg_list):
                possibilities.append(gears_copy)
        sum += len(possibilities)
    print(sum)

def run_b(input: str):
    sum = 0
    REPEAT = 5
    cache = {}
    for li, line in enumerate(input):
        gears, dmg_report = line.rstrip().split(" ")
        gears = "?".join([g for g in gears.split(",")] * REPEAT)
        dmg_list = [int(d) for d in dmg_report.split(",")] * REPEAT
        cache.clear()
        sum += find_arrangements(cache, gears, dmg_list, 0, 0, 0)
    print(sum)

def find_arrangements(cache: dict[(int, int, int): int], gears: list[str], dmg_list: list[int], gi: int, di: int, dl: int):
    key = (gi, di, dl)
    if key in cache:
        return cache[key]

    # a valid arrangement is found if the entire gears list has been parsed along with the end of the damaged gears list
    # or parsing is past the end of the damaged gears list with no more damaged gears to track
    if gi == len(gears):
        return 1 if (di == len(dmg_list) and dl == 0) or ((di == len(dmg_list)-1) and dmg_list[di] == dl) else 0

    arrangements_found = 0
    for c in ["#", "."]:
        if gears[gi] in [c, "?"]:
            # recurse this run of damaged gears
            if c == "#":
                arrangements_found += find_arrangements(cache, gears, dmg_list, gi+1, di, dl+1)
            elif c == ".":
                # advance over working gear if not parsing a run of damaged
                if dl == 0:
                    arrangements_found += find_arrangements(cache, gears, dmg_list, gi+1, di, 0)
                # found expected end of damaged list element before end of gears
                elif di < len(dmg_list) and dmg_list[di] == dl:
                    arrangements_found += find_arrangements(cache, gears, dmg_list, gi+1, di+1, 0)

    # memoize the number of arrangements for this gears/damaged/consecutive damaged combination
    cache[key] = arrangements_found
    return arrangements_found

def evaluate_possibility_a(gears: list[str], dmg_list: list[int]):
    RE_A = "\\.+".join([f"#{{{d}}}" for d in dmg_list])
    RE_A = "\\A\\.*" + f"{RE_A}\\.*" + "\\Z"
    m = regex.match(RE_A, "".join(gears))
    return m is not None

#  used by abandoned brute force attempt in part 2
def evaluate_possibility_b(gears: str, dmg_list: list[int], repeat: int):
    RE_A = "\\.+".join([f"#{{{d}}}" for d in dmg_list])
    RE_A = "\\A\\.*" + f"({RE_A}\\.*){{{repeat}}}" + "\\Z"
    m = regex.match(RE_A, gears)
    return m is not None
