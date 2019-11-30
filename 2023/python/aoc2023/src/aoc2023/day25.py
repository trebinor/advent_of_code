from collections import defaultdict
import networkx as nx

def run_a(input: list[str]):
    network = nx.Graph()
    hash = defaultdict(list)
    for l in input:
        line = l.strip().split(": ")
        hash[line[0]] = set([p.strip() for p in line[1].split(" ")])

    for k, v in hash.items():
        for val in v:
            network.add_edge(k, val)

    cut_count = 0
    for mod in nx.minimum_edge_cut(network):
        # print(f"Removing {mod[0]} from {mod[1]}")
        network.remove_edge(mod[0], mod[1])
        cut_count += 1
    assert cut_count == 3, "Exactly 3 cuts required!"

    connected = [c for c in nx.connected_components(network)]
    assert len(connected) == 2, "Exactly 2 separately connected networks expected!"

    print(len(connected[0]) * len(connected[1]))
