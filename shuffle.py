#!/usr/bin/python3
import sys
import random
from itertools import product

suites = ["♥️", "♠️", "♣️", "♦️"]
suite_names = {
    "♥️": "H",
    "♠️": "S",
    "♣️": "C",
    "♦️": "D"
}
nums = []
nums.extend([str(i) for i in range(2, 11)])
nums.extend(["J", "Q", "K", "A"])
values = {nums[i]: i for i in range(13)}

deck = list(product(suites, nums))


def sort_key(card):
    return (card[0], values[card[1]])


seed = int(sys.argv[1])
num_cards = int(sys.argv[2])
turn = int(sys.argv[3])

random.seed(seed)
random.shuffle(deck)

my_cards = deck[num_cards * (turn - 1):num_cards * turn]
my_cards.sort(key=sort_key)

print("Suite    Card")
for card in my_cards:
    print("{} ({})    {}".format(card[0], suite_names[card[0]], card[1]))
