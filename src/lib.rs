#![feature(local_key_cell_methods)]
use std::cell::RefCell;

use topcodes::Scanner;
use wasm_bindgen::prelude::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

thread_local! {
    static TOPCODE_SCANNER: RefCell<Scanner> = RefCell::new(Scanner::new(WIDTH, HEIGHT));
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // let window = web_sys::window().expect("No global `window` exists");
    // let document = window.document().expect("Should have a document on window");
    // let body = document.body().expect("Document should have a body");

    // TODO: Move all/most of the DOM work into wasm to reduce JS required for demo.

    Ok(())
}

#[wasm_bindgen(js_name = "getTopcodes")]
pub fn get_topcodes(buffer: &js_sys::Uint32Array) -> js_sys::Array {
    let topcodes = TOPCODE_SCANNER.with_borrow_mut(|scanner| {
        return scanner.scan(buffer, |buffer, index| {
            let index = index as u32;
            let pixel = buffer.get_index(index);
            ((pixel >> 16) & 0xff, (pixel >> 8) & 0xff, pixel & 0xff)
        });
    });

    return topcodes
        .into_iter()
        .map(|code| JsValue::from_str(&code.to_json()))
        .collect::<js_sys::Array>();
}
