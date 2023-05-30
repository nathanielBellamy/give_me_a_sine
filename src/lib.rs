use crate::wasm::Wasm;
use wasm_bindgen::prelude::*;

mod wasm;
mod ui_manifest;

#[wasm_bindgen]
pub struct GmasWasm;

#[wasm_bindgen]
impl GmasWasm {
    pub fn run() -> JsValue {
        Wasm::run()
    }
}
