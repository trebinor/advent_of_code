import math
import regex

RE_A = regex.compile(r"([A-Z]*) = \(([A-Z]*), ([A-Z]*)")

def run_a(input: str):
    directions = input[0].rstrip()
    instructions = dict()
    for l in input[2:]:
        m = regex.findall(RE_A, l)
        instructions[m[0][0]] = (m[0][1], m[0][2])

    state = "AAA"
    count = 0
    break_out = False
    while not break_out:
        for s in directions:
            if s == "L":
                # print(f"State L {state} -> {instructions[state][0]}")
                state = instructions[state][0]
            elif s == "R":
                # print(f"State R {state} -> {instructions[state][1]}")
                state = instructions[state][1]
            else:
                raise Exception("Invalid direction")
            count += 1
            if state == 'ZZZ':
                break_out = True
                break
    print(count)

def run_b(input: str):
    directions = input[0].rstrip()
    instructions = dict()
    for l in input[2:]:
        m = regex.findall(RE_A, l)
        instructions[m[0][0]] = (m[0][1], m[0][2])

    # find all initial states that end in A
    states = []
    for i in instructions:
        if i[2] == "A":
            states.append(i)

    count = 0
    break_out = False
    lcm = dict()
    while not break_out:
        for d in directions:
            lr = 0 if d == "L" else 1
            for i,s in enumerate(states):
                states[i] = instructions[s][lr]
            count += 1
            for i,s in enumerate(states):
                if s[2] == "Z":
                    if f"{s}@{i}" not in lcm:
                        lcm[f"{s}@{i}"] = count
                        # print(f"{s}@{i} in {count}")
            if len(lcm) == len(states):
                break_out = True
                break
    print(math.lcm(*lcm.values()))
