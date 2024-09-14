LDI r0 200 # Load immediate '200' into register r0
LDI r1 55
ADD r2 r1 r0
SB r1 r2 r3
LB r3 r0 r1

LDI r0 200
LDI r1 55
ADD r2 r1 r0 # Should set ZF (Zero Flag)

LDI r5 55
LDI r6 55
JNZ r5 r6 # This jump should not be executed

LDI r1 0
ADD r2 r1 r0 # Should clear ZF

LDI r5 99
LDI r6 99
JNZ r5 r6 # Should jump to segment [99:99]

HLT
