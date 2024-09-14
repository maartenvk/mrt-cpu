# ISA
Read [the sheet](ISA.ods)

# Instruction encoding
Variant **NoParam**:
```
 0    3 4    7
[opcode]------
```

Variant **RegImm**:
```
 0    3  4  7   8           15
[opcode][reg1] [immediate     ]
```

Variant **DoubleReg**:
```
 0    3  4  7   8   11  12  15
[opcode][reg1] [reg2  ]--------
```

Variant **DoubleRegImm4**:
```
 0    3  4  7   8   11  12  15
[opcode][reg1] [reg2  ] [imm4 ] 
```

Variant **TripleReg**:
```
 0    3  4  7   8   11  12  15
[opcode][reg1] [reg2  ][reg3  ]
```
