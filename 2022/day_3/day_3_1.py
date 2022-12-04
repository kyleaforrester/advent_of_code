#!/usr/bin/env python3

import sys

alphabet = ' abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
priorities = {}

for i in range(1, len(alphabet)):
    priorities[alphabet[i]] = i

def sack_priority(sack):
    pivot = int(len(sack)/2)
    comp_1 = set(sack[:pivot])
    comp_2 = set(sack[pivot:])

    return priorities[(comp_1 & comp_2).pop()]


lines = sys.stdin.read().splitlines()

print(sum(map(sack_priority, lines)))

