from dataclasses import dataclass


def run_a(input: str):
    sum = 0
    for l in input:
        card = l.split(": ")
        nums = card[1].split("|")
        winning = [int(w) for w in nums[0].strip().split(" ") if w != ""]
        have = [int(h) for h in nums[1].strip().rstrip().split(" ") if h != ""]
        sum += score_card(winning, have)
    print(sum)

def run_b(input: str):
    cards = []
    for i, l in enumerate(input):
        card = l.split(": ")
        id = card[0].split(" ")[1]
        nums = card[1].split("|")
        winning = [int(w) for w in nums[0].strip().split(" ") if w != ""]
        have = [int(h) for h in nums[1].strip().rstrip().split(" ") if h != ""]
        cards.append(Card(winning, have, 1))
    for i, c in enumerate(cards):
        for copy_index in range(c.score_card()):
            cards[copy_index+i+1].num += cards[i].num
    print(sum([c.num for c in cards]))

def score_card(winning: list[int], have: list[int]):
    matching = len(set(winning) & set(have))
    return 2**(matching - 1) if matching > 0 else 0

@dataclass
class Card:
    winning: list[int]
    have: list[int]
    num: int

    def score_card(self) -> int:
        return len(set(self.winning) & set(self.have))
