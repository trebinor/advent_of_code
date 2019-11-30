import regex

RE_A = regex.compile(r"[0-9]")
RE_B = regex.compile(r"one|two|three|four|five|six|seven|eight|nine|[0-9]")


def run_a(input: str):
    sum = 0
    for l in input:
        m = regex.findall(RE_A, l)
        sum += 10 * int(m[0]) + int(m[-1])
    print(sum)


def run_b(input: str):
    sum = 0
    for l in input:
        m = regex.findall(RE_B, l, overlapped=True)
        c = (num_to_int(m[0]), num_to_int(m[-1]))
        sum += 10 * c[0] + c[1]
    print(sum)


def num_to_int(num: str):
    if num == "one":
        return 1
    if num == "two":
        return 2
    if num == "three":
        return 3
    if num == "four":
        return 4
    if num == "five":
        return 5
    if num == "six":
        return 6
    if num == "seven":
        return 7
    if num == "eight":
        return 8
    if num == "nine":
        return 9
    return int(num)
