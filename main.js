import init, {
    initialize_rom,
    run
}
from "./pkg/chip_8_wasm.js";

let data = null;
let ctx = document.getElementById("canvas").getContext("2d");
canvas.style.backgroundColor = "black";
ctx.clearRect(0, 0, 640, 320);

// Instantiate wasm module
await init("./pkg/chip_8_wasm_bg.wasm");

const runWasm = async() => {

    

    canvas.style.backgroundColor = "black";
    ctx.clearRect(0, 0, 640, 320);
    

    initialize_rom(data);
    let displayArr = run();
    console.log(displayArr);

    for (let i = 0; i < 32 * 64; i++) {
        let x = i % 64;
        let y = Math.floor(i / 64);

        ctx.fillStyle = "white";
        if (displayArr[i]) {
            ctx.fillRect(x * 10, y * 10, 10, 10);
        }
    }

};

let rom = null;

const fileSelector = document.getElementById('rom-select');
fileSelector.addEventListener('change', (event) => {
    rom = event.target.files[0];
    let reader = new FileReader();
    reader.onload = function (e) {
        // binary data
        data = new Uint8Array(e.target.result);
        runWasm();
    };
    reader.onerror = function (e) {
        // error occurred
        console.log('Error : ' + e.type);
    };
    reader.readAsArrayBuffer(rom);
});
