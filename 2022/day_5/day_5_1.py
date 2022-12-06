#!/usr/bin/env python3

import sys

sections = sys.stdin.read().split('\n\n')

header = sections[0].splitlines()
stack_names = header[-1]
stack_indices = {}

for i in range(len(stack_names)):
    if stack_names[i] != ' ':
        stack_indices[i] = stack_names[i]
    

stacks = {}
crates = sections[0].splitlines()[:-1]
for l in reversed(crates):
    for i in range(len(l)):
        if l[i] not in ' []':
            if stack_indices[i] in stacks:
                stacks[stack_indices[i]].append(l[i])
            else:
                stacks[stack_indices[i]] = [l[i]]

for l in sections[1].splitlines():
    splits = l.split()

    amount = int(splits[1])
    prev = splits[3]
    to = splits[5]

    for i in range(amount):
        stacks[to].append(stacks[prev].pop())

for pairs in sorted(list(stacks.items()), key=lambda x: x[0]):
    print(pairs[1].pop(), end='')
print()

