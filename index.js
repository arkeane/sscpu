import init, { run_full, run_step, init_state } from "./pkg/sscpu.js";

await init();

function run() {
    const program_text = document.getElementById("program").value;
    const data_text = document.getElementById("data").value;

    // convert each line first 16 chars of program_text to a number from binary to decimal
    const program = program_text.split("\n").map(line => parseInt(line.substring(0, 16), 2));
    // convert each line first 16 chars of data_text to a number from binary to decimal
    const data = data_text.split("\n").map(line => parseInt(line.substring(0, 16), 2));

    var JsValue = run_full(program, data);
    // send the json registers to registers textarea
    document.getElementById("registers").value = JSON.stringify(JsValue.registers);
    // send the json memory to memory textarea
    document.getElementById("memory").value = JSON.stringify(JsValue.memory);
    // send the json stack to stack textarea
    document.getElementById("stack").value = JSON.stringify(JsValue.stack);
    // send the json pc to pc textarea
    document.getElementById("pc").value = JSON.stringify(JsValue.pc);
    // send the json sp to sp textarea
    document.getElementById("sp").value = JSON.stringify(JsValue.sp);
}

function initcpu() {
    const program_text = document.getElementById("program").value;
    const data_text = document.getElementById("data").value;

    // convert each line first 16 chars of program_text to a number from binary to decimal
    const program = program_text.split("\n").map(line => parseInt(line.substring(0, 16), 2));
    // convert each line first 16 chars of data_text to a number from binary to decimal
    const data = data_text.split("\n").map(line => parseInt(line.substring(0, 16), 2));

    var JsValue = init_state(program, data);
    // send the json registers to registers textarea
    document.getElementById("registers").value = JSON.stringify(JsValue.registers);
    // send the json memory to memory textarea
    document.getElementById("memory").value = JSON.stringify(JsValue.memory);
    // send the json stack to stack textarea
    document.getElementById("stack").value = JSON.stringify(JsValue.stack);
    // send the json pc to pc textarea
    document.getElementById("pc").value = JSON.stringify(JsValue.pc);
    // send the json sp to sp textarea
    document.getElementById("sp").value = JSON.stringify(JsValue.sp);
}

function runstep() {
    const registers = document.getElementById("registers");
    const memory = document.getElementById("memory");
    const stack = document.getElementById("stack");
    const pc = document.getElementById("pc");
    const sp = document.getElementById("sp");

    // create json from previous vars
    const cpustate = {
        registers: JSON.parse(registers.value),
        memory: JSON.parse(memory.value),
        stack: JSON.parse(stack.value),
        pc: JSON.parse(pc.value),
        sp: JSON.parse(sp.value)
    };

    var JsValue = run_step(cpustate);

    // send the json registers to registers textarea
    document.getElementById("registers").value = JSON.stringify(JsValue.registers);
    // send the json memory to memory textarea
    document.getElementById("memory").value = JSON.stringify(JsValue.memory);
    // send the json stack to stack textarea
    document.getElementById("stack").value = JSON.stringify(JsValue.stack);
    // send the json pc to pc textarea
    document.getElementById("pc").value = JSON.stringify(JsValue.pc);
    // send the json sp to sp textarea
    document.getElementById("sp").value = JSON.stringify(JsValue.sp);
}

function load_from_file() {
    // open file
    const file = document.getElementById("ssasm").files[0];
    const reader = new FileReader();
    // split file into program and data using --data-- as a delimiter
    reader.onload = function (e) {
        const text = e.target.result;
        const split = text.split("--data--");
        document.getElementById("program").value = split[0];
        // delete last line from program as it is empty
        document.getElementById("program").value = document.getElementById("program").value.substring(0, document.getElementById("program").value.length - 1);
        document.getElementById("data").value = split[1];
        // delete first line from data
        document.getElementById("data").value = document.getElementById("data").value.substring(document.getElementById("data").value.indexOf("\n") + 1);
    }
    reader.readAsText(file);
}

// download a file from server and load it into the program and data textareas
function load_from_server() {
    const request = new XMLHttpRequest();
    request.open("GET", "./ssasm/conditional.ssasm", true);
    request.onload = function (e) {
        const text = e.target.response;
        const split = text.split("--data--");
        document.getElementById("program").value = split[0];
        // delete last line from program as it is empty
        document.getElementById("program").value = document.getElementById("program").value.substring(0, document.getElementById("program").value.length - 1);
        document.getElementById("data").value = split[1];
        // delete first line from data
        document.getElementById("data").value = document.getElementById("data").value.substring(document.getElementById("data").value.indexOf("\n") + 1);
    }
    request.send();
}

document.getElementById("run_full").addEventListener("click", run);
document.getElementById("run_step").addEventListener("click", runstep);
document.getElementById("init").addEventListener("click", initcpu);
document.getElementById("example_load").addEventListener("click", load_from_server);
document.getElementById("ssasm").addEventListener("change", load_from_file);