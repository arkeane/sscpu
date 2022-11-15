use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

}

#[derive(Serialize, Deserialize)]
pub struct Cpu {
    pub registers: [u16; 8], // 8 registers
    pub memory: Vec<u16>,    // memory
    pub stack: Vec<u16>,     // stack
    pub pc: u16,             // program counter
    pub sp: u16,             // stack pointer
}

// reset the cpu to its initial state
pub fn reset_cpu() -> Cpu {
    Cpu {
        registers: [0; 8],
        memory: [0; 4096].to_vec(),
        stack: [0; 16].to_vec(),
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
    // k is next 4 bits
    let k = (instruction >> 8) & 0b1111;
    // r0 is next 4 bits
    let a = (instruction >> 4) & 0b1111;
    // r1 is last 4 bits
    let b = instruction & 0b1111;

    [opcode, k, a, b]
}

// Instruction Set implementation

// push R0 to stack
// 0001
fn push(cpu: &mut Cpu, r0: u16) {
    if cpu.sp == 16 {
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
    cpu.stack[cpu.sp as usize] = 0;
}

// move R0 to R1
// 0011
fn mov(cpu: &mut Cpu, r0: u16, r1: u16) {
    cpu.registers[r0 as usize] = cpu.registers[r1 as usize];
}

// jump to address
// 0100
fn jmp(cpu: &mut Cpu, k: u16, a: u16, b: u16) {
    // create a 12 bit u32 address with k, a and b
    let address = k << 8 | a << 4 | b;
    cpu.pc = address;
}

// branch if R0 is Equal to R1
// 0101
fn beq(cpu: &mut Cpu, offset: u16, r0: u16, r1: u16) {
    if cpu.registers[r0 as usize] == cpu.registers[r1 as usize] {
        cpu.pc += offset;
    }
}

// branch if R0 is not Equal to R1
// 0110
fn bne(cpu: &mut Cpu, offset: u16, r0: u16, r1: u16) {
    if cpu.registers[r0 as usize] != cpu.registers[r1 as usize] {
        cpu.pc += offset;
    }
}

// load data from one of first 255 locations in memory
// 0111
fn stor(cpu: &mut Cpu, r0: u16, a: u16, b: u16) {
    // create a 8 bit u32 address with a and b
    let address = (a as u32) << 4 | (b as u32);
    cpu.memory[address as usize] = cpu.registers[r0 as usize];
}

// load data from one of first 255 locations in memory
// 1000
fn load(cpu: &mut Cpu, r0: u16, a: u16, b: u16) {
    // create a 8 bit u32 address with a and b
    let address = (a as u32) << 4 | (b as u32);
    cpu.registers[r0 as usize] = cpu.memory[address as usize];
}

// add R0 to R1 and store in R2
// 1001
fn add(cpu: &mut Cpu, r0: u16, r1: u16, r2: u16) {
    cpu.registers[r2 as usize] = cpu.registers[r0 as usize] + cpu.registers[r1 as usize];
}

// subtract R1 from R0 and store in R2
// 1010
fn sub(cpu: &mut Cpu, r0: u16, r1: u16, r2: u16) {
    let a = cpu.registers[r0 as usize];
    let b = cpu.registers[r1 as usize];
    if b > a {
        cpu.registers[r2 as usize] = 0;
        return;
    }
    cpu.registers[r2 as usize] = (a - b) as u16;
}

// multiply R0 with R1 and store in R2
// 1011
fn mul(cpu: &mut Cpu, r0: u16, r1: u16, r2: u16) {
    cpu.registers[r2 as usize] = cpu.registers[r0 as usize] * cpu.registers[r1 as usize];
}

// bitwise AND R0 with R1 and store in R2
// 1100
fn and(cpu: &mut Cpu, r0: u16, r1: u16, r2: u16) {
    cpu.registers[r2 as usize] = cpu.registers[r0 as usize] & cpu.registers[r1 as usize];
}

// bitwise OR R0 with R1 and store in R2
// 1101
fn or(cpu: &mut Cpu, r0: u16, r1: u16, r2: u16) {
    cpu.registers[r2 as usize] = cpu.registers[r0 as usize] | cpu.registers[r1 as usize];
}

// NOT R0 and store in R1
// 1110
fn not(cpu: &mut Cpu, r0: u16, r1: u16) {
    cpu.registers[r1 as usize] = !cpu.registers[r0 as usize];
}

fn nop(_cpu: &mut Cpu) {}

pub fn decode_and_execute(cpu: &mut Cpu) -> bool {
    let [opcode, k, a, b] = split_instruction(cpu);

    match opcode {
        0b0000 => return false,
        0b0001 => push(cpu, a),
        0b0010 => pop(cpu, b),
        0b0011 => mov(cpu, a, b),
        0b0100 => jmp(cpu, k, a, b),
        0b0101 => beq(cpu, k, a, b),
        0b0110 => bne(cpu, k, a, b),
        0b0111 => stor(cpu, k, a, b),
        0b1000 => load(cpu, k, a, b),
        0b1001 => add(cpu, k, a, b),
        0b1010 => sub(cpu, k, a, b),
        0b1011 => mul(cpu, k, a, b),
        0b1100 => and(cpu, k, a, b),
        0b1101 => or(cpu, k, a, b),
        0b1110 => not(cpu, a, b),
        0b1111 => nop(cpu),
        _ => alert(&format!("Unknown opcode: {}", opcode)),
    }

    cpu.pc += 1;
    true
}

pub fn send_cpu_to_js(cpu: &Cpu) -> JsValue {
    JsValue::from_serde(cpu).unwrap()
}

pub fn get_cpu_from_js(jscpu: JsValue) -> Cpu {
    jscpu.into_serde().unwrap()
}
