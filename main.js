import init, {
    greet
}
from "./pkg/chip_8_wasm.js";
init().then(() => {
    greet("WebAssembly");
});
