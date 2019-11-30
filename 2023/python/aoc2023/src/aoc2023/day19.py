import regex
import math
RE_WORKFLOW = regex.compile(r"(.*)\{(.*)\}")
RE_PART = regex.compile(r"x=([0-9]*),m=([0-9]*),a=([0-9]*),s=([0-9]*)")
RE_NEW_WORKFLOW = regex.compile(r"\A([a-z]*)\Z")
RE_WORKFLOW_ITEM = regex.compile(r"([a-z])(\>|\<)([0-9]*):([A-Za-z]*)")


def run_a(input: list[str]):
    workflows = {}
    parts = []
    newline_parsed = False
    for li in input:
        if newline_parsed:
            part_line = li.rstrip().strip("{}")
            xmas = regex.match(RE_PART, part_line)
            parts.append({prop: int(xmas[i+1]) for i, prop in enumerate("xmas")})

        else:
            if li == "\n":
                newline_parsed = True
            else:
                workflow_line = li.rstrip()
                m = regex.match(RE_WORKFLOW, workflow_line)
                workflows[m[1]] = m[2].split(",")

    sum = 0
    for p in parts:
        if process_workflows(workflows, "in", p):
            # print(f"pw true, sum += {p.sum()}")
            sum += sum_part(p)
    print(sum)


def run_b(input: list[str]):
    workflows = {}
    parts = []
    newline_parsed = False
    for li in input:
        if newline_parsed:
            part_line = li.rstrip().strip("{}")
            xmas = regex.match(RE_PART, part_line)
            parts.append({prop: int(xmas[i+1]) for i, prop in enumerate("xmas")})

        else:
            if li == "\n":
                newline_parsed = True
            else:
                workflow_line = li.rstrip()
                m = regex.match(RE_WORKFLOW, workflow_line)
                workflows[m[1]] = m[2].split(",")

    accepted_parts = process_workflows_b(workflows)
    print()
    for p in accepted_parts:
        print(p)
    print(sum([math.prod([part[prop][1] - part[prop][0] + 1 for prop in "xmas"]) for part in accepted_parts]))

def sum_part(p: dict[str, int]) -> int:
    return sum(p.values())

def process_workflows(workflows: dict[str, list[str]], workflow_name: str, p: dict[str, int]) -> bool:
    # print(f"Part {p} entering workflow {workflows[workflow_name]}")
    for step in workflows[workflow_name]:
        if step == "A":
            print(f"Adding {p} as final step in workflow {workflow_name}")
            return True
        if step == "R":
            return False
        m = regex.match(RE_NEW_WORKFLOW, step)
        if m:
            # print(f"new workflow {m[1]}")
            return process_workflows(workflows, m[1], p)
        if ">" in step or "<" in step:
            m = regex.match(RE_WORKFLOW_ITEM, step)
            if (m[2] == ">" and p[m[1]] > int(m[3])) or (m[2] == "<" and p[m[1]] < int(m[3])):
                if m[4] == "A":
                    print(f"Adding {p} as intermediate step in workflow {workflow_name}")
                    return True
                if m[4] == "R":
                    return False
                return process_workflows(workflows, m[4], p)

def process_workflows_b(workflows: dict[str, list[str]]) -> list[int]:
    workflow_steps = [("in", {prop: (1, 4000) for prop in "xmas"})]
    accepted = []
    while len(workflow_steps) > 0:
        step, part = workflow_steps.pop()
        print(f"popped step={step} part={part}")
        if step == "A":
            print(part)
            accepted.append(part)
        elif step == "R":
            pass
        else:
            print(f"Processing step={step} wf={workflows[step]}")
            for w in workflows[step]:
                print(w)
                if w == "R":
                    continue
                elif w == "A":
                    workflow_steps.append((w, part))
                    continue
                m = regex.match(RE_NEW_WORKFLOW, w)
                if m:
                    workflow_steps.append((m[1], part.copy()))
                else:
                    m = regex.match(RE_WORKFLOW_ITEM, w)
                    print(m[1], m[2], m[3], m[4])
                    if m[2] == ">":
                        prop = part[m[1]]
                        part[m[1]] = (max(prop[0], int(m[3]))+1, prop[1])
                        print(f"Appending step {m[4]} {part}")
                        workflow_steps.append((m[4], part.copy()))
                        part[m[1]] = (prop[0], min(prop[1], int(m[3])))
                    elif m[2] == "<":
                        prop = part[m[1]]
                        part[m[1]] = (prop[0], min(prop[1], int(m[3]))-1)
                        print(f"Appending step {m[4]} {part}")
                        workflow_steps.append((m[4], part.copy()))
                        part[m[1]] = (max(prop[0], int(m[3])), prop[1])
                    else:
                        assert False
    return accepted
