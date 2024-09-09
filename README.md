# ISA
Read [the sheet](ISA.ods)

# Instruction encoding
Variant **NoParam**:
```
 0    3 4    7
[opcode]------
```

Variant **ImmReg**:
```
 0    3  4  7   8           15
[opcode][reg1] [immediate     ]
```

Variant **TripleReg**:
```
 0    3  4  7   8   11  12  15
[opcode][reg1] [reg2  ][reg3  ]
```
