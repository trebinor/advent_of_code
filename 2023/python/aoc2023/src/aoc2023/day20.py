from collections import deque

def run_a(input: str):
    conjunctions = {}
    flipflops = {}
    network = {}
    for line in input:
        mod, outputs = line.strip().split(" -> ")
        outputs = [o.strip() for o in outputs.split(",")]
        if mod == "broadcaster":
            # skip so that it's added to the network along with its targets
            pass
        elif mod[0] == "%":
            # flip-flops init to low pulse
            flipflops[mod[1:]] = False
        elif mod[0] == "&":
            conjunctions[mod[1:]] = {}
        else:
            assert False, f"Unhandled module {mod} encounted"

        if mod == "broadcaster":
            network[mod] = outputs
        else:
            network[mod[1:]] = outputs

    # print(network)

    # init conjunctions to remember low pulses for all inputs
    for input, outputs in network.items():
        for output in outputs:
            if output in conjunctions:
                conjunctions[output][input] = False

    sum_low = 0
    sum_high = 0

    PRESSES = 1000
    for i in range(PRESSES):
        pulses_deq = deque([("button", "broadcaster", False)])
        while len(pulses_deq) > 0:
            input, output, pulse_high = pulses_deq.popleft()
            # print(f"Processing {input} {output} {pulse_high}")
            sum_low, sum_high = sum_low if pulse_high else sum_low + 1, sum_high + 1 if pulse_high else sum_high
            if output in flipflops:
                if pulse_high:
                    continue
                else:
                    next_pulse = flipflops[output] = not flipflops[output]
            elif output in conjunctions:
                conjunctions[output][input] = pulse_high
                next_pulse = not all(conjunctions[output].values())
            elif output in network:
                next_pulse = pulse_high
            else:
                continue

            for next_output in network[output]:
                pulses_deq.append((output, next_output, next_pulse))

    print(sum_low * sum_high)

def run_b(input: str):
    conjunctions = {}
    flipflops = {}
    network = {}
    for line in input:
        mod, outputs = line.strip().split(" -> ")
        outputs = [o.strip() for o in outputs.split(",")]
        if mod == "broadcaster":
            # skip so that it's added to the network along with its targets
            pass
        elif mod[0] == "%":
            # flip-flops init to low pulse
            flipflops[mod[1:]] = False
        elif mod[0] == "&":
            conjunctions[mod[1:]] = {}
        else:
            assert False, f"Unhandled module {mod} encounted"

        if mod == "broadcaster":
            network[mod] = outputs
        else:
            network[mod[1:]] = outputs

    # print(network)

    # init conjunctions to remember low pulses for all inputs
    for input, outputs in network.items():
        for output in outputs:
            if output in conjunctions:
                conjunctions[output][input] = False

    sum_low = 0
    sum_high = 0

    presses = 0
    # use the following loop to detect cycle frequencies. There are 4 per the input.
    # the number of button presses for 1 rx low pulse is then math.lcm([cycle_value + 1 for cycle_value in cycle_values_determined_by_loop])
    while True:
        pulses_deq = deque([("button", "broadcaster", False)])
        while len(pulses_deq) > 0:
            input, output, pulse_high = pulses_deq.popleft()
            # print(presses, output)
            if output == "ft" and any(conjunctions[output].values()) and pulse_high:
                print(f"ft high with {[(key, value) for key, value in conjunctions['ft'].items()]} at {presses}")
            # if output in ["ft"] and not pulse_high:
                # print(f"output {output} high at {presses}")
            # print(f"Processing {input} {output} {pulse_high}")
            sum_low, sum_high = sum_low if pulse_high else sum_low + 1, sum_high + 1 if pulse_high else sum_high
            if output in flipflops:
                if pulse_high:
                    continue
                else:
                    next_pulse = flipflops[output] = not flipflops[output]
            elif output in conjunctions:
                conjunctions[output][input] = pulse_high
                next_pulse = not all(conjunctions[output].values())
            elif output in network:
                next_pulse = pulse_high
            else:
                continue

            for next_output in network[output]:
                pulses_deq.append((output, next_output, next_pulse))
        presses += 1

    print(sum_low * sum_high)
