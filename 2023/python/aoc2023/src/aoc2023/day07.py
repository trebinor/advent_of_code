from dataclasses import dataclass
from itertools import permutations
import copy

def run_a(input: str):
    hands = []
    for l in input:
        line = l.rstrip().split(" ")
        hand = []
        for card in line[0]:
            if card == "T":
                hand.append(10)
            elif card == "J":
                hand.append(11)
            elif card == "Q":
                hand.append(12)
            elif card == "K":
                hand.append(13)
            elif card == "A":
                hand.append(14)
            else:
                hand.append(int(card))
        hands.append(Hand(hand, int(line[1])))

    high = []
    kind2 = []
    pair2 = []
    kind3 = []
    fh = []
    kind4 = []
    kind5 = []

    for hand in hands:
        break_out = False
        s = hand.cards
        if s[0] == s[1] and s[1] == s[2] and s[2] == s[3] and s[3] == s[4]:
            kind5.append(hand)
        else:
            for p in permutations(s):
                if p[0] == p[1] and p[1] == p[2] and p[2] == p[3]:
                    kind4.append(hand)
                    break_out = True
                    break
            if not break_out:
                for p in permutations(s):
                    if p[0] == p[1] and p[1] == p[2] and p[3] == p[4]:
                        fh.append(hand)
                        break_out = True
                        break
            if not break_out:
                for p in permutations(s):
                    if p[0] == p[1] and p[1] == p[2]:
                        kind3.append(hand)
                        break_out = True
                        break
            if not break_out:
                for p in permutations(s):
                    if p[0] == p[1] and p[2] == p[3]:
                        pair2.append(hand)
                        break_out = True
                        break
            if not break_out:
                for p in permutations(s):
                    if p[0] == p[1]:
                        kind2.append(hand)
                        break_out = True
                        break
            if not break_out:
                high.append(hand)

    for cardset in [high, kind2, pair2, kind3, fh, kind4, kind5]:
        cardset.sort(key=lambda hand: hand.get_score())

    ranked = high + kind2 + pair2 + kind3 + fh + kind4 + kind5
    # for i, r in enumerate(ranked):
        # print(f"{str(r.cards):20} {r.bid:4} {r.get_score()} {i+1}")
    print(sum([((i+1)*hand.bid) for i,hand in enumerate(ranked)]))

def run_b(input: str):
    hands = []
    for l in input:
        line = l.rstrip().split(" ")
        hand = []
        for card in line[0]:
            if card == "T":
                hand.append(10)
            elif card == "J":
                hand.append(1)
            elif card == "Q":
                hand.append(11)
            elif card == "K":
                hand.append(12)
            elif card == "A":
                hand.append(13)
            else:
                hand.append(int(card))
        hands.append(Hand(hand, int(line[1])))

    high = []
    kind2 = []
    pair2 = []
    kind3 = []
    fh = []
    kind4 = []
    kind5 = []

    for h, hand in enumerate(hands):
        break_out = False
        s = copy.copy(hand.cards)
        js = [s]
        jokered = False
        for j in range(2, 14): # replace J with 2 - A
            joker_hand = copy.copy(s)
            for i, c in enumerate(s):
                if c == 1: # joker
                    jokered = True
                    joker_hand[i] = j
            if jokered:
                js.append(copy.copy(joker_hand))
        for j in js:
            for p in permutations(j):
                if p[0] == p[1] and p[1] == p[2] and p[2] == p[3] and p[3] == p[4]:
                    kind5.append(hand)
                    break_out = True
                    break
            if break_out:
                break
        for j in js:
            if not break_out:
                for p in permutations(j):
                    if p[0] == p[1] and p[1] == p[2] and p[2] == p[3]:
                        kind4.append(hand)
                        break_out = True
                        break
            if break_out:
                break
        for j in js:
            if not break_out:
                for p in permutations(j):
                    if p[0] == p[1] and p[1] == p[2] and p[3] == p[4]:
                        fh.append(hand)
                        break_out = True
                        break
            if break_out:
                break
        for j in js:
            if not break_out:
                for p in permutations(j):
                    if p[0] == p[1] and p[1] == p[2]:
                        kind3.append(hand)
                        break_out = True
                        break
            if break_out:
                break
        for j in js:
            if not break_out:
                for p in permutations(j):
                    if p[0] == p[1] and p[2] == p[3]:
                        pair2.append(hand)
                        break_out = True
                        break
            if break_out:
                break
        for j in js:
            if not break_out:
                for p in permutations(j):
                    if p[0] == p[1]:
                        kind2.append(hand)
                        break_out = True
                        break
            if break_out:
                break
        if not break_out:
            high.append(hand)

    for cardset in [high, kind2, pair2, kind3, fh, kind4, kind5]:
        cardset.sort(key=lambda hand: hand.get_score())

    ranked = high + kind2 + pair2 + kind3 + fh + kind4 + kind5
    # for i, r in enumerate(ranked):
        # print(f"{str(r.cards):20} {r.bid:4} {r.get_score()} {i+1}")
    print(sum([((i+1)*hand.bid) for i,hand in enumerate(ranked)]))

@dataclass
class Hand:
    cards: list[int]
    bid: int

    def get_score(self) -> int:
        return self.cards[0] * 10**10 + self.cards[1] * 10**8 + self.cards[2] * 10**6 + self.cards[3] * 10**4 + self.cards[4] * 10**2

