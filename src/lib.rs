use isa::{get_cpu_from_js, send_cpu_to_js};
use wasm_bindgen::prelude::*;
mod isa;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn opcode_name(opcode: u16) -> &'static str {
    match opcode {
        0b0000 => "HALT",
        0b0001 => "PUSH",
        0b0010 => "POP",
        0b0011 => "MOV",
        0b0100 => "JMP",
        0b0101 => "BEQ",
        0b0110 => "BNE",
        0b0111 => "STOR",
        0b1000 => "LOAD",
        0b1001 => "ADD",
        0b1010 => "SUB",
        0b1011 => "MUL",
        0b1100 => "AND",
        0b1101 => "OR",
        0b1110 => "NOT",
        0b1111 => "NOP",
        _ => "unknown",
    }
}

// print the current state of the cpu
fn print_state(cpu: &isa::Cpu) {
    let [opcode, k, a, b] = isa::split_instruction(cpu);
    log(&format!(
        "Instruction:      Opcode: {:04x}({:?}), k: {:04b}, a: {:04b}, b: {:04b}",
        opcode,
        opcode_name(opcode),
        k,
        a,
        b
    ));
    log(&format!("Register          {:?}", cpu.registers));
    log(&format!("Program Counter:  {:?}", cpu.pc));
    log(&format!("Stack Pointer:    {:?}", cpu.sp));
}

fn run(cpu: &mut isa::Cpu, program: Vec<u16>, data: Vec<u16>) {
    isa::load_instructions(cpu, &program);
    isa::load_data(cpu, &data);
    loop {
        if !isa::decode_and_execute(cpu) {
            break;
        }
    }
}

// test cpu with one instruction
#[wasm_bindgen]
pub fn sanity_check() {
    let program: Vec<u16> = [36928].to_vec();
    let data: Vec<u16> = [].to_vec();
    let mut cpu = isa::reset_cpu();
    log("Starting CPU");
    isa::load_instructions(&mut cpu, &program);
    log("Loaded instructions");
    isa::load_data(&mut cpu, &data);
    log("Loaded data");
    loop {
        print_state(&cpu);
        if !isa::decode_and_execute(&mut cpu) {
            break;
        }
    }
    log("CPU halted");
}

#[wasm_bindgen]
// function that takes two vec<u16> as instructions and data
pub fn run_full(program: Vec<u16>, data: Vec<u16>) -> JsValue {
    let mut cpu = isa::reset_cpu();
    run(&mut cpu, program, data);
    send_cpu_to_js(&cpu)
}

#[wasm_bindgen]
pub fn init_state(program: Vec<u16>, data: Vec<u16>) -> JsValue {
    let mut cpu = isa::reset_cpu();
    isa::load_instructions(&mut cpu, &program);
    isa::load_data(&mut cpu, &data);
    send_cpu_to_js(&cpu)
}

#[wasm_bindgen]
pub fn run_step(jscpu: JsValue) -> JsValue {
    let mut cpu = get_cpu_from_js(jscpu);
    isa::decode_and_execute(&mut cpu);
    send_cpu_to_js(&cpu)
}
