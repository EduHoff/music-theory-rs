#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use music_theory_rs::app::run_app;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run_app()
}
