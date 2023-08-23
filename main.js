import init, {
    initialize_rom
}
from "./pkg/chip_8_wasm.js";

let data = null;

const runWasm = async() => {

    console.log(data);
    // Instantiate wasm module

    await init("./pkg/chip_8_wasm_bg.wasm");

    console.log(data.length);
    initialize_rom(data);

};

let rom = null;

const fileSelector = document.getElementById('rom-select');
fileSelector.addEventListener('change', (event) => {
    rom = event.target.files[0];
    let reader = new FileReader();
    reader.onload = function (e) {
        // binary data
        let utf8Encode = new TextEncoder();
        data = utf8Encode.encode(e.target.result);
        runWasm();
    };
    reader.onerror = function (e) {
        // error occurred
        console.log('Error : ' + e.type);
    };
    reader.readAsBinaryString(rom);
});
