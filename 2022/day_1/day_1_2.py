#!/usr/bin/env python3

import sys

per_elf_text = sys.stdin.read().split('\n\n')

per_elf_text = [l.strip() for l in per_elf_text]

elf_totals = map(lambda text_block: sum(map(int, text_block.split('\n'))), per_elf_text)

elf_totals = sorted(elf_totals, reverse=True)

print(sum(elf_totals[:3]))
