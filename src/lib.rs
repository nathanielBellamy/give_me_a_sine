use wasm_bindgen::prelude::*;
use crate::wasm::Wasm;

mod wasm;

#[wasm_bindgen]
pub struct GmasWasm;

#[wasm_bindgen]
impl GmasWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() {
        Wasm::run();
    }
}
