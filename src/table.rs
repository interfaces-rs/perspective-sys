use crate::view::{View, ViewConfig};
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;

pub type TableConfig = Object;

#[wasm_bindgen(module = perspective)]
extern {
    pub type Table;

    #[wasm_bindgen(method)]
    pub fn delete(this: &Table);

    #[wasm_bindgen(method)]
    pub fn on_delete(this: &Table, cb: &Function);

    #[wasm_bindgen(method)]
    pub fn size(this: &Table) -> Promise;

    #[wasm_bindgen(method)]
    pub fn schema(this: &Table) -> Promise;

    #[wasm_bindgen(method)]
    pub fn computed_schema(this: &Table) -> Promise;

    #[wasm_bindgen(method)]
    pub fn view(this: &Table, config: Option<&ViewConfig>) -> View;

    #[wasm_bindgen(method)]
    pub fn update(this: &Table, data: &JsValue);

    #[wasm_bindgen(method)]
    pub fn remove(this: &Table, data: Box<[JsValue]>);

    #[wasm_bindgen(method)]
    pub fn columns(this: &Table) -> Array;

    #[wasm_bindgen]
    pub fn table(data: &JsValue, options: Option<&TableConfig>) -> Table;
}
