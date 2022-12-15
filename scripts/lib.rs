#![allow(non_upper_case_globals)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["console"])]
    fn log(message: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    log("Hello World!");
}
