import init from "./pkg/chip_8_wasm.js";
const runWasm = async() => {
    // Instantiate our wasm module
    const chip8 = await init("./pkg/chip_8_wasm_bg.wasm");

    chip8.greet();

};

runWasm();
