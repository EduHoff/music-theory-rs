use crate::core::audio::AudioCalculator;
use crate::core::audio_player::AudioPlayer;
use slint::{ComponentHandle, Model, Timer};
use std::error::Error;
use std::time::Duration;

slint::include_modules!();

pub mod circle_view;
pub mod harmonic_field_view;
pub mod scale_view;

pub fn run_app() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    scale_view::setup(&ui);
    circle_view::setup(&ui);
    harmonic_field_view::setup(&ui);

    let player = AudioPlayer::new();

    let player_note = player.clone();
    let ui_handle_note = ui.as_weak();
    ui.on_play_note(move |note_str| {
        let Some(ui) = ui_handle_note.upgrade() else {
            return;
        };
        let current_root = ui.get_selected_root().to_string();

        let clean_note = note_str.to_string();
        let base_note = clean_note.split('/').next().unwrap_or(&clean_note).trim();

        let hz = AudioCalculator::note_to_hz_relative(&current_root, base_note);
        player_note.play_freq(hz, 400);

        let notes_model = ui.get_scale_notes();
        for i in 0..notes_model.row_count() {
            if let Some(mut item) = notes_model.row_data(i) {
                let item_note = item.note.to_string();
                let clean_item = item_note.split('/').next().unwrap_or(&item_note).trim();
                if clean_item == base_note {
                    item.is_active = true;
                    notes_model.set_row_data(i, item);

                    let ui_weak = ui.as_weak();
                    Timer::single_shot(Duration::from_millis(300), move || {
                        if let Some(ui) = ui_weak.upgrade() {
                            let model = ui.get_scale_notes();
                            if let Some(mut reset_item) = model.row_data(i) {
                                reset_item.is_active = false;
                                model.set_row_data(i, reset_item);
                            }
                        }
                    });
                    break;
                }
            }
        }
    });

    let player_scale = player.clone();
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

        let steps = AudioCalculator::build_scale_sequence(&notes_vec);
        if steps.is_empty() {
            return;
        }

        let ui_weak = ui.as_weak();
        player_scale.play_steps(steps, move |active_idx| {
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

    let player_chord = player.clone();
    let ui_handle_chord = ui.as_weak();
    ui.on_play_chord(move |card_idx, notes_str| {
        let Some(ui) = ui_handle_chord.upgrade() else {
            return;
        };

        let freqs = AudioCalculator::chord_to_hz(&notes_str);
        player_chord.play_chord(&freqs, 600);

        if let Ok(idx) = usize::try_from(card_idx) {
            let chords_model = ui.get_harmonic_chords();
            if let Some(mut item) = chords_model.row_data(idx) {
                item.is_active = true;
                chords_model.set_row_data(idx, item);

                let ui_weak = ui.as_weak();
                Timer::single_shot(Duration::from_millis(550), move || {
                    if let Some(ui) = ui_weak.upgrade() {
                        let model = ui.get_harmonic_chords();
                        if let Some(mut reset_item) = model.row_data(idx) {
                            reset_item.is_active = false;
                            model.set_row_data(idx, reset_item);
                        }
                    }
                });
            }
        }
    });

    let player_harmonic = player;
    let ui_handle_harmonic = ui.as_weak();
    ui.on_play_harmonic_sequence(move || {
        let Some(ui) = ui_handle_harmonic.upgrade() else {
            return;
        };

        let chords_model = ui.get_harmonic_chords();
        let mut chords_notes = Vec::new();

        for i in 0..chords_model.row_count() {
            if let Some(item) = chords_model.row_data(i) {
                chords_notes.push(item.notes_string.to_string());
            }
        }

        let steps = AudioCalculator::build_harmonic_sequence(&chords_notes);
        if steps.is_empty() {
            return;
        }

        let ui_weak = ui.as_weak();
        player_harmonic.play_chord_steps(steps, move |active_idx| {
            if let Some(ui) = ui_weak.upgrade() {
                let model = ui.get_harmonic_chords();
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
