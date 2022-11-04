# SSCPU
This is a proof of concept ISA with an emulator. It is not intended to be complete or completely working.

## CPU
- 16-bit Instruction Set
- 12-bit Memory Addressing      (4096 16-bit Programs starting at 256)
- 8-bit Data Addressing         (256 16-bit Data Addresses starting at 0 ending at 255)
- 16-bit Stack                  (16 16-bit Addresses starting at 0 ending at 255)
- 16-bit Registers              (8 General Purpose, 1 Program Counter, 1 Stack Pointer)