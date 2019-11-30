import math

def run_a(input: str):
    times = [int(t) for t in input[0].split(":")[1].rstrip().split(" ") if t != ""]
    distances = [int(d) for d in input[1].split(":")[1].rstrip().split(" ") if d != ""]
    record_breakers = {d:[] for d in distances}
    for record in zip(times, distances):
        for i in range(record[0]+1):
            time = i*(record[0]-i)
            if time > record[1]:
                record_breakers[record[1]].append(time)
    print(math.prod([len(rb) for rb in record_breakers.values()]))

def run_b(input: str):
    record_breakers = 0
    time = int(input[0].split(":")[1].rstrip().replace(" ", ""))
    record = int(input[1].split(":")[1].rstrip().replace(" ", ""))
    for i in range(time+1):
        t = i*(time-i)
        if t > record:
            record_breakers += 1
    print(record_breakers)

