LDI r0 0xFF

LDI r0 245
LDI r1 34

XOR r0 r1 r0
XOR r1 r0 r1
XOR r0 r1 r0

SUB r2 r1 r0

SHL r0 r0 2
SHR r1 r1 2

# r0, r1 should be swapped.
# r2 should be 245 - 34

LDI r5 0
LDI r6 128
ADD r5 r6 r6

LDI r7 0
AND r6 r6 r7 # r6 should be zero

LDI r8 170
LDI r9 85
OR r8 r8 r9 # r8 should be 255

LDI r10 170
NOT r10 r10 # r10 should be 85

LDI r9 99
JC r9 r9

HLT

LDI r2 0 # 0 extend
LDI r3 8

LDI r0 0 
LDI r1 1

LDI r5 0
LDI r6 17
JAL r4 r5 r6
JNZ r2 r3

HLT

ADD r0 r0 r1
LDI r6 128
ADD r6 r6 r0 
JNZ r4 r5

HLT
