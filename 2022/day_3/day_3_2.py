#!/usr/bin/env python3

import sys

alphabet = ' abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
priorities = {}

for i in range(1, len(alphabet)):
    priorities[alphabet[i]] = i

def group_priority(group):
    badge = (set(group[0]) & set(group[1]) & set(group[2])).pop()
    return priorities[badge]


lines = sys.stdin.read().splitlines()

print(sum(map(group_priority, zip(lines[::3], lines[1::3], lines[2::3]))))
