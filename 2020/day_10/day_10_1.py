
import sys

fd = sys.stdin.readlines()

fd = [int(l.strip()) for l in fd]

fd.append(0)

fd.sort()

ones = 0
threes = 1
for i in range(0, len(fd)-1):
    diff = fd[i+1] - fd[i]
    if diff == 1:
        ones += 1
    elif diff == 3:
        threes += 1

print(ones * threes)
