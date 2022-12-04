#!/usr/bin/env python3

import sys

rounds = sys.stdin.readlines()

outcomes = [[3, 0, 6],
            [6, 3, 0],
            [0, 6, 3]]

round_sum = 0
for r in rounds:
    them = r[0]
    us = r[2]

    if them == 'A':
        them_idx = 0
    elif them == 'B':
        them_idx = 1 
    else:
        them_idx = 2

    if us == 'X':
        us_idx = 0
    elif us == 'Y':
        us_idx = 1 
    else:
        us_idx = 2

    round_sum += outcomes[us_idx][them_idx] + us_idx + 1

print(round_sum)
