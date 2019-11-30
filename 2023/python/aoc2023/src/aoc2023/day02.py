import regex

RE_A = regex.compile(r"([0-9]* red)|([0-9]* blue)|([0-9]* green)")


def run_a(input: str):
    sum = 0
    for l in input:
        game_grabs = l.split(": ")
        id = game_grabs[0].split(" ")[1]
        grabs = game_grabs[1].split("; ")
        if is_game_possible(grabs, num_r=12, num_g=13, num_b=14):
            sum += int(id)
    print(sum)


def run_b(input: str) -> int:
    sum = 0
    for i, l in enumerate(input):
        game_grabs = l.split(": ")
        grabs = game_grabs[1].split("; ")
        sum += get_power(grabs)
    print(sum)


def is_game_possible(grabs, num_r: int, num_g: int, num_b: int):
    for g in grabs:
        match = regex.findall(RE_A, g)
        for m in match:
            for n in m:
                if n == "":
                    continue
                nc = n.split(" ")
                if (
                    (nc[1] == "red" and num_r < int(nc[0]))
                    or (nc[1] == "green" and num_g < int(nc[0]))
                    or (nc[1] == "blue" and num_b < int(nc[0]))
                ):
                    return False
    return True


def get_power(grabs) -> int:
    r, g, b = 0, 0, 0
    for grab in grabs:
        match = regex.findall(RE_A, grab)
        for m in match:
            for n in m:
                if n == "":
                    continue
                nc = n.split(" ")
                if nc[1] == "red":
                    r = max(r, int(nc[0]))
                if nc[1] == "green":
                    g = max(g, int(nc[0]))
                if nc[1] == "blue":
                    b = max(b, int(nc[0]))
    return r * g * b
