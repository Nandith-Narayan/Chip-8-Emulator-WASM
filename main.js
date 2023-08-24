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

    let displayArr = run(10, buttonStatus);

    for (let i = 0; i < 32 * 64; i++) {
        let x = i % 64;
        let y = Math.floor(i / 64);

        ctx.fillStyle = "white";
        if (displayArr[i]) {
            ctx.fillRect(x * 10, y * 10, 10, 10);
        }
    }

    requestAnimationFrame(runWasm);

};

let buttonStatus = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false];
let buttonKeyCodes = [88, 49, 50, 51, 81, 87, 69, 65, 83, 68, 90, 67, 52, 82, 70, 86];
let buttons = []
for (let i = 0; i < 16; i++) {
    let buttonId = "button" + (i.toString(16)).toUpperCase();
    buttons.push(document.getElementById(buttonId));
}

document.body.addEventListener("keydown", function (e) {
    for (let i = 0; i < 16; i++) {
        if (e.keyCode == buttonKeyCodes[i]) {
            buttons[i].classList.add("active");
            buttonStatus[i] = true;
        }
    }

});
document.body.addEventListener("keyup", function (e) {
    for (let i = 0; i < 16; i++) {
        if (e.keyCode == buttonKeyCodes[i]) {
            buttons[i].classList.remove("active");
            buttonStatus[i] = false;
        }
    }

});

let rom = null;

const fileSelector = document.getElementById('rom-select');
fileSelector.addEventListener('change', (event) => {
    rom = event.target.files[0];
    let reader = new FileReader();
    reader.onload = function (e) {
        // binary data
        data = new Uint8Array(e.target.result);
        initialize_rom(data);
        runWasm();
    };
    reader.onerror = function (e) {
        // error occurred
        console.log('Error : ' + e.type);
    };
    reader.readAsArrayBuffer(rom);
});
