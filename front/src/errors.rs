use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}
