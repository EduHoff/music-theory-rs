pub mod app;
pub mod core;

slint::include_modules!();

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    let main_window = app::AppWindow::new().unwrap();
    app::scale_view::setup(&main_window);
    main_window.run().unwrap();
}

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub fn web_main() -> Result<(), JsValue> {
    let main_window = app::AppWindow::new().map_err(|e| JsValue::from_str(&e.to_string()))?;
    app::scale_view::setup(&main_window);
    main_window
        .run()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}
