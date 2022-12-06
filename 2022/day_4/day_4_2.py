#!/usr/bin/env python3

import sys

pairs = sys.stdin.readlines()

pairs = [l.split(',') for l in pairs]

pairs = [(l[0].split('-'), l[1].split('-')) for l in pairs]

pairs = [((int(l[0][0]), int(l[0][1])), (int(l[1][0]), int(l[1][1]))) for l in pairs]

pairs = [l for l in pairs if l[0][0] <= l[1][1] and l[0][1] >= l[1][0]]

print(len(pairs))
