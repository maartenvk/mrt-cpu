# Mrt-8 CPU
Word size 8-bit, addressing size 16-bit.
Harvard architecture.

Planning to migrate to Von Neumann architecture, and increasing word size to 16/32-bit.

This is fully written in the Rust programming language.  
It is a Command Line Interface in which you may compile custom assembly files into .rom files.

Such rom file can be loaded into the system emulator using the `load_rom` command.

# ISA
Read [the sheet](ISA.ods)

RISC based instruction set architecture

# Instruction encoding
Variant **NoParam**:
```
 0    3 4    7
[opcode]------
```

Variant **RegImm**:
```
 0    3   4   7   8           15
[opcode] [reg1 ] [imm8          ]
```

Variant **DoubleReg**:
```
 0    3   4   7   8  11  12   15
[opcode] [reg1 ] [reg2 ]---------
```

Variant **DoubleRegImm4**:
```
 0    3   4   7   8  11   12  15
[opcode] [reg1 ] [reg2 ] [imm4  ] 
```

Variant **TripleReg**:
```
 0    3   4   7   8  11   12  15
[opcode] [reg1 ] [reg2 ] [reg3  ]
```
