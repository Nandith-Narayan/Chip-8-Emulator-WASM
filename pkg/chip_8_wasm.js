import * as wasm from "./chip_8_wasm_bg.wasm";
import { __wbg_set_wasm } from "./chip_8_wasm_bg.js";
__wbg_set_wasm(wasm);
export * from "./chip_8_wasm_bg.js";
