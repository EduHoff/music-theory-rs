use crate::core::audio::AudioManager;
use slint::{ComponentHandle, Model};
use std::error::Error;
use std::sync::Arc;

slint::include_modules!();

pub mod scale_view;

pub fn run_app() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    scale_view::setup(&ui);

    let audio_manager = Arc::new(AudioManager::new().expect("Failed to initialize AudioManager"));

    let audio_note_mgr = Arc::clone(&audio_manager);
    let ui_handle_note = ui.as_weak();
    ui.on_play_note(move |note_str| {
        let Some(ui) = ui_handle_note.upgrade() else {
            return;
        };
        let current_root = ui.get_selected_root().to_string();

        let clean_note = note_str.to_string();
        let base_note = clean_note.split('/').next().unwrap_or(&clean_note).trim();

        let hz = AudioManager::note_to_hz_relative(&current_root, base_note);
        audio_note_mgr.play_freq(hz, 400);

        let notes_model = ui.get_scale_notes();
        for i in 0..notes_model.row_count() {
            if let Some(mut item) = notes_model.row_data(i) {
                let item_note = item.note.to_string();
                let clean_item = item_note.split('/').next().unwrap_or(&item_note).trim();
                if clean_item == base_note {
                    item.is_active = true;
                    notes_model.set_row_data(i, item);

                    let ui_weak = ui.as_weak();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(300));
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_weak.upgrade() {
                                let model = ui.get_scale_notes();
                                if let Some(mut reset_item) = model.row_data(i) {
                                    reset_item.is_active = false;
                                    model.set_row_data(i, reset_item);
                                }
                            }
                        });
                    });
                    break;
                }
            }
        }
    });

    let audio_scale_mgr = Arc::clone(&audio_manager);
    let ui_handle_scale = ui.as_weak();
    ui.on_play_scale_sequence(move || {
        let Some(ui) = ui_handle_scale.upgrade() else {
            return;
        };
        let notes_model = ui.get_scale_notes();

        let mut notes_vec = Vec::new();
        for i in 0..notes_model.row_count() {
            if let Some(item) = notes_model.row_data(i) {
                let note_str = item.note.to_string();
                let clean = note_str
                    .split('/')
                    .next()
                    .unwrap_or(&note_str)
                    .trim()
                    .to_string();
                notes_vec.push(clean);
            }
        }

        if notes_vec.is_empty() {
            return;
        }

        let ui_weak = ui.as_weak();
        audio_scale_mgr.play_scale_sequence(notes_vec, move |active_idx| {
            if let Some(ui) = ui_weak.upgrade() {
                let model = ui.get_scale_notes();
                for i in 0..model.row_count() {
                    if let Some(mut item) = model.row_data(i) {
                        let should_be_active = Some(i) == active_idx;
                        if item.is_active != should_be_active {
                            item.is_active = should_be_active;
                            model.set_row_data(i, item);
                        }
                    }
                }
            }
        });
    });

    ui.run()?;
    Ok(())
}
