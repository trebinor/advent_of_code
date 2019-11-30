def run_a(input: str):
    codes = input[0].rstrip().split(",")
    sum = 0
    for code in codes:
        sum += hash(code)
    print(sum)

def run_b(input: str):
    codes = input[0].rstrip().split(",")
    boxes = [{} for b in range(256)]
    for code in codes:
        box = hash(code[:-1]) if code[-1] == "-" else hash(code[:-2])
        if code[-1] == '-':
            if code[:-1] in boxes[box]:
                del boxes[box][code[:-1]]
        elif code[-2] == '=':
            boxes[box][(code[:-2])] = code[-1]

    sum = 0
    for b, box in enumerate(boxes):
        for li, lens in enumerate(box):
            sum += (b+1) * (li+1) * int(box[lens][-1])
    print(sum)

def hash(code):
    current = 0
    for c in code:
        current += ord(c)
        current *= 17
        current %= 256
    return current

