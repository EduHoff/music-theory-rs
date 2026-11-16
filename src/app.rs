use std::error::Error;

slint::include_modules!();

pub mod scale_view;

pub fn run_app() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    scale_view::setup(&ui);

    ui.run()?;
    Ok(())
}
