# Loading
LDI r0 0x00
LDI r1 72 # H
LDI r2 69 # E
LDI r3 76 # L
LDI r4 79 # O
LDI r5 32 # ' '
LDI r6 87 # W
LDI r7 82 # R
LDI r8 68 # D

# Write out
# [0 << 8 | 0]
# Serial OUT memory mapped to [0]

# HELLO
SB r1 r0 r0
SB r2 r0 r0
SB r3 r0 r0
SB r3 r0 r0
SB r4 r0 r0

# ' '
SB r5 r0 r0

# WORLD
SB r6 r0 r0
SB r4 r0 r0
SB r7 r0 r0
SB r3 r0 r0
SB r8 r0 r0

HLT
