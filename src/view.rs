use js_sys::Object;
use wasm_bindgen::prelude::*;

pub type AggregateConfig = Object;

pub type ViewConfig = Object;

#[wasm_bindgen(module = perspective)]
extern {
    pub type View;
}
