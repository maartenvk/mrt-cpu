LDI r2 0 # 0 extend
LDI r3 8

LDI r0 0 
LDI r1 1

# lives at ip=8, see r3
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
