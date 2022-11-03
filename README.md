# SSCPU
This is a proof of concept ISA with an emulator. It is not intended to be complete or completely working.

## CPU
- 16-bit Instruction Set
- 12-bit Memory Addressing      (4096 16-bit Programs starting at 256)
- 8-bit Data Addressing         (256 16-bit Data Addresses starting at 0 ending at 255)
- 16-bit Stack                  (256 16-bit Addresses starting at 0 ending at 255)
- 16-bit Registers              (8 General Purpose, 1 Program Counter, 1 Stack Pointer)

## Instruction Set
16-bit Instructions

| OPCODE | OPT            | FROM          | TO            | INSTRUCTION | COMMENT                                                                                                     |
| ------ | -------------- | ------------- | ------------- | ----------- | ----------------------------------------------------------------------------------------------------------- |
| 0000   | 0000           | 0000          | 0000          | HALT        | Stops the CPU                                                                                               |
| 0001   | 0000           | R             | 0000          | PUSH        | Pushes the value of register R onto the stack, if stack pointer is = 256 Stack Overflow is thrown.          |
| 0010   | 0000           | 0000          | R             | POP         | Pops the value of the stack into registe R, if stack pointer is = 0 Stack Out of Bound is thrown            |
| 0011   | 0000           | R1            | R0            | MOV         | Moves the value of register R1 into register R0                                                             |
| 0100   | 12-bit Address |               |               | JMP         | Jumps to the address specified by the 12-bit address                                                        |
| 0101   | 4-bit Offset  | R1            | R0            | BEQ         | Branches to the spcial address if R0 == R1                                                                  |
| 0110   | 4-bit Offset  | R1            | R0            | BNE         | Branches to the spcial address if R0 != R1                                                                  |
| 0111   | R0             |               | 8-bit Address | STOR        | Stores the value of R0 into the 8-bit address in Memory                                                     |
| 1000   | R0             | 8-bit Address |               | LOAD        | Loads the value of the 8-bit address in Memory into R0                                                      |
| 1001   | 0000           | A             | R0            | ADDI        | Adds the value of A to the value of R0 and stores it in R0                                                  |
| 1010   | 0000           | R1            | R0            | ADDR        | Adds the value of R1 to the value of R0 and stores it in R0                                                 |
| 1011   | 0000           | A             | R0            | SUBI        | Subtracts the value of A from the value of R0 and stores it in R0 if A is greater than R0, R0 is set to 0   |
| 1100   | 0000           | R1            | R0            | SUBR        | Subtracts the value of R1 from the value of R0 and stores it in R0 if R1 is greater than R0, R0 is set to 0 |
| 1101   | 0000           | A             | R0            | MULI        | Multiplies the value of A to the value of R0 and stores it in R0                                            |
| 1110   | 0000           | R1            | R0            | MULR        | Multiplies the value of R1 to the value of R0 and stores it in R0                                           |
| 1111   | 0000           | 0000          | 0000          | NOP         | No Operation PC+1                                                                                           |


