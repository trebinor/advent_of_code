import regex
import numpy as np
import math

RE_A = regex.compile(r"([0-9]+)")


def run_a(input: str):
    sum = 0
    machine = np.array([np.array(l.rstrip()) for l in input])
    for row, _ in enumerate(machine):
        for match in regex.finditer(RE_A, machine[row]):
            for n in range(match.start(), match.end()):
                if (
                    (row > 0 and n > 0 and is_symbol(machine[row - 1][n - 1]))
                    or (
                        row > 0
                        and n < len(machine[0]) - 1
                        and is_symbol(machine[row - 1][n])
                    )
                    or (
                        row > 0
                        and n < len(machine[0]) - 2
                        and is_symbol(machine[row - 1][n + 1])
                    )
                    or (n > 0 and is_symbol(machine[row][n - 1]))
                    or (n < len(machine[0]) - 2 and is_symbol(machine[row][n + 1]))
                    or (
                        row < len(machine) - 2
                        and n > 0
                        and is_symbol(machine[row + 1][n - 1])
                    )
                    or (
                        row < len(machine) - 2
                        and n < len(machine[0]) - 1
                        and is_symbol(machine[row + 1][n])
                    )
                    or (
                        row < len(machine) - 2
                        and n < len(machine[0]) - 2
                        and is_symbol(machine[row + 1][n + 1])
                    )
                ):
                    sum += int(match.group(1))
                    break
    print(sum)

def run_b(input: str):
    sum = 0
    machine = np.array([np.array(l.rstrip()) for l in input])
    for row, text in enumerate(machine):
        for cell, ctext in enumerate(text):
            if machine[row][cell] != "*":
                continue

            g = []
            if machine[row-1][cell].isdigit():
                g.append(find_gear_mul(row-1, cell, machine))
            else:
                if machine[row-1][cell-1].isdigit():
                    g.append(find_gear_mul(row-1, cell-1, machine))
                    if len(g) == 2:
                        sum += math.prod(g)
                        continue
                if machine[row-1][cell+1].isdigit():
                    g.append(find_gear_mul(row-1, cell+1, machine))
                    if len(g) == 2:
                        sum += math.prod(g)
                        continue

            if machine[row][cell-1].isdigit():
                g.append(find_gear_mul(row, cell-1, machine))
                if len(g) == 2:
                    sum += math.prod(g)
                    continue
            if machine[row][cell+1].isdigit():
                g.append(find_gear_mul(row, cell+1, machine))
                if len(g) == 2:
                    sum += math.prod(g)
                    continue

            if machine[row+1][cell].isdigit():
                g.append(find_gear_mul(row+1, cell, machine))
                if len(g) == 2:
                    sum += math.prod(g)
                    continue
            else:
                if machine[row+1][cell-1].isdigit():
                    g.append(find_gear_mul(row+1, cell-1, machine))
                    if len(g) == 2:
                        sum += math.prod(g)
                        continue
                if machine[row+1][cell+1].isdigit():
                    g.append(find_gear_mul(row+1, cell+1, machine))
                    if len(g) == 2:
                        sum += math.prod(g)
                        continue

    print(sum)

def find_gear_mul(row: int, cell: int, machine: np.ndarray) -> int:
    for match in regex.finditer(RE_A, machine[row]):
        if cell in range(match.start(), match.end()):
            return int(match.group(1))
    return 0

def is_symbol(b: chr) -> bool:
    return (ord(b) != ord(".")) and (ord(b) < ord("0")) or (ord(b) > ord("9"))
