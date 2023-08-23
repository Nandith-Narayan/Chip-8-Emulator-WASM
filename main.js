import init from "./pkg/chip_8_wasm.js";

let data = null;

const runWasm = async() => {
    
    console.log(data);
    // Instantiate wasm module
    
    //data = [1,2,3,4,5];
    const chip8 = await init("./pkg/chip_8_wasm_bg.wasm");
    console.log(data.length);
    chip8.greet(data, data.length);

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


