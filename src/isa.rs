
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

pub struct Cpu {
    pub registers: [u16; 8], // 8 registers
    pub memory: [u16; 4096], // memory
    pub stack: [u16; 256],   // stack
    pub pc: u16,             // program counter
    pub sp: u16,             // stack pointer
}

// reset the cpu to its initial state
pub fn reset_cpu() -> Cpu {
    Cpu {
        registers: [0; 8],
        memory: [0; 4096],
        stack: [0; 256],
        pc: 256,
        sp: 0,
    }
}

// load instructions in memory starting from position pc=200
pub fn load_instructions(cpu: &mut Cpu, instructions: &[u16]) {
    for (i, instruction) in instructions.iter().enumerate() {
        cpu.memory[256 + i] = *instruction;
    }
}

// load data in memory from address 0 to 255
pub fn load_data(cpu: &mut Cpu, data: &[u16]) {
    for (i, datum) in data.iter().enumerate() {
        cpu.memory[i] = *datum;
        if i == 255 {
            break;
        }
    }
}

// split instruction into opcode and operands
pub fn split_instruction(cpu: &Cpu) -> [u16; 4] {
    let instruction = cpu.memory[cpu.pc as usize];
    // opcode is first 4 bits
    let opcode = instruction >> 12;
    // option is next 4 bits
    let option = (instruction >> 8) & 0b1111;
    // r0 is next 4 bits
    let a = (instruction >> 4) & 0b1111;
    // r1 is last 4 bits
    let b = instruction & 0b1111;

    [opcode, option, a, b]
}

// Instruction Set implementation

// push R0 to stack
// 0001
fn push(cpu: &mut Cpu, r0: u16) {
    if cpu.sp == 256 {
        alert("Stack overflow");
    }
    cpu.stack[cpu.sp as usize] = cpu.registers[r0 as usize];
    cpu.sp += 1;
}

// pop R0 from stack
// 0010
fn pop(cpu: &mut Cpu, r0: u16) {
    if cpu.sp == 0 {
        alert("Stack underflow");
    }
    cpu.sp -= 1;
    cpu.registers[r0 as usize] = cpu.stack[cpu.sp as usize];
}

// move R1 to R0
// 0011
fn mov(cpu: &mut Cpu, r1: u16, r0: u16) {
    cpu.registers[r0 as usize] = cpu.registers[r1 as usize];
}

// jump to address
// 0100
fn jmp(cpu: &mut Cpu, option: u16, a: u16, b: u16) {
    // create a 12 bit u32 address with option, a and b
    let address = option << 8 | a << 4 | b;
    cpu.pc = address;
}

// branch if R0 is Equal to R1
// 0101
fn beq(cpu: &mut Cpu, option: u16, r1: u16, r0: u16) {
    if cpu.registers[r0 as usize] == cpu.registers[r1 as usize] {
        cpu.pc += option;
    }
}

// branch if R0 is Equal to R1
// 0110
fn bne(cpu: &mut Cpu, option: u16, r1: u16, r0: u16) {
    if cpu.registers[r0 as usize] != cpu.registers[r1 as usize] {
        cpu.pc += option;
    }
}

// load data from one of first 255 locations in memory
// 0111
fn stor(cpu: &mut Cpu, option: u16, a: u16, b: u16) {
    // create a 8 bit u32 address with a and b
    let address = (a as u32) << 4 | (b as u32);
    cpu.memory[address as usize] = cpu.registers[option as usize];
}

// load data from one of first 255 locations in memory
// 1000
fn load(cpu: &mut Cpu, option: u16, a: u16, b: u16) {
    // create a 8 bit u32 address with a and b
    let address = (a as u32) << 4 | (b as u32);
    cpu.registers[option as usize] = cpu.memory[address as usize];
}

// add A to R0
// 1001
fn addi(cpu: &mut Cpu, a: u16, r0: u16) {
    cpu.registers[r0 as usize] += a;
}

// add R1 to R0 and store in R0
// 1010
fn addr(cpu: &mut Cpu, r1: u16, r0: u16) {
    cpu.registers[r0 as usize] += cpu.registers[r1 as usize];
}

// subtract R1 from R0 and store in R0
// 1011
fn subi(cpu: &mut Cpu, a: u16, r0: u16) {
    let b = cpu.registers[r0 as usize];
    if a > b {
        cpu.registers[r0 as usize] = 0;
        return;
    }
    let c = b - a;
    cpu.registers[r0 as usize] = c;
}

// subtract R1 from R0 and store in R0
// 1100
fn subr(cpu: &mut Cpu, r1: u16, r0: u16) {
    let b = cpu.registers[r0 as usize];
    let a = cpu.registers[r1 as usize];
    if a > b {
        cpu.registers[r0 as usize] = 0;
        return;
    }
    let c = b - a;
    cpu.registers[r0 as usize] = c;
}

// multiply A with R0 and store in R0
// 1101
fn muli(cpu: &mut Cpu, a: u16, r0: u16) {
    cpu.registers[r0 as usize] *= a;
}

// multiply R1 with R0 and store in R0
// 1110
fn mulr(cpu: &mut Cpu, r1: u16, r0: u16) {
    cpu.registers[r0 as usize] *= cpu.registers[r1 as usize];
}

fn nop(_cpu: &mut Cpu) {}

pub fn decode_and_execute(cpu: &mut Cpu) -> bool {
    let [opcode, option, a, b] = split_instruction(cpu);

    match opcode {
        0b0000 => return false,
        0b0001 => push(cpu, a),
        0b0010 => pop(cpu, b),
        0b0011 => mov(cpu, a, b),
        0b0100 => jmp(cpu, option, a, b),
        0b0101 => beq(cpu, option, a, b),
        0b0110 => bne(cpu, option, a, b),
        0b0111 => stor(cpu, option, a, b),
        0b1000 => load(cpu, option, a, b),
        0b1001 => addi(cpu, a, b),
        0b1010 => addr(cpu, a, b),
        0b1011 => subi(cpu, a, b),
        0b1100 => subr(cpu, a, b),
        0b1101 => muli(cpu, a, b),
        0b1110 => mulr(cpu, a, b),
        0b1111 => nop(cpu),
        _ => alert(&format!("Unknown opcode: {}", opcode)),
    }

    cpu.pc += 1;
    true
}
