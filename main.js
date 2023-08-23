import init, { greet } from "./pkg/hello_wasm.js";
      init().then(() => {
        greet("WebAssembly");
      });