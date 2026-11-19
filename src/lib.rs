pub mod app;
pub mod core;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    app::run_app().unwrap();
}

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub fn web_main() -> Result<(), JsValue> {
    app::run_app().map_err(|e| JsValue::from_str(&e.to_string()))
}
