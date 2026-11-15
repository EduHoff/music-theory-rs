use crate::app::{AppWindow, ScaleNoteItem};
use crate::core::note::Note;
use crate::core::scale::{Scale, ScaleType};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::rc::Rc;

const ROMAN_DEGREES: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
const CHROMATIC_DEGREES: [&str; 12] = [
    "P1", "m2", "M2", "m3", "M3", "P4", "trit", "P5", "m6", "M6", "m7", "M7",
];

fn format_chromatic_note(note_str: &str) -> String {
    match note_str {
        "C#" => "C# / Db".to_string(),
        "Db" => "C# / Db".to_string(),
        "D#" => "D# / Eb".to_string(),
        "Eb" => "D# / Eb".to_string(),
        "F#" => "F# / Gb".to_string(),
        "Gb" => "F# / Gb".to_string(),
        "G#" => "G# / Ab".to_string(),
        "Ab" => "G# / Ab".to_string(),
        "A#" => "A# / Bb".to_string(),
        "Bb" => "A# / Bb".to_string(),
        other => other.to_string(),
    }
}

pub fn setup(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    let scale_names: Vec<SharedString> = ScaleType::ALL
        .iter()
        .map(|st| SharedString::from(st.name()))
        .collect();
    ui.set_available_scales(ModelRc::from(Rc::new(VecModel::from(scale_names))));

    let refresh = move || {
        let Some(ui) = ui_handle.upgrade() else {
            return;
        };
        let root_str = ui.get_selected_root();
        let scale_idx = ui.get_selected_scale_idx();

        let root_note = root_str
            .parse::<Note>()
            .unwrap_or_else(|_| Note::new(crate::core::note::Letter::C, 0));
        let scale_type = ScaleType::from_index(scale_idx);
        let generated = Scale::generate(root_note, scale_type);

        let items: Vec<ScaleNoteItem> = generated
            .notes
            .iter()
            .enumerate()
            .map(|(i, note)| {
                let degree = if scale_type == ScaleType::Chromatic {
                    CHROMATIC_DEGREES.get(i).copied().unwrap_or("-")
                } else if generated.notes.len() == 7 {
                    ROMAN_DEGREES.get(i).copied().unwrap_or("-")
                } else {
                    match i {
                        0 => "1st",
                        1 => "2nd",
                        2 => "3rd",
                        3 => "4th",
                        4 => "5th",
                        5 => "6th",
                        6 => "7th",
                        7 => "8th",
                        _ => "-",
                    }
                };

                let note_text = if scale_type == ScaleType::Chromatic {
                    format_chromatic_note(&note.to_string())
                } else {
                    note.to_string()
                };

                ScaleNoteItem {
                    degree: SharedString::from(degree),
                    note: SharedString::from(note_text),
                }
            })
            .collect();

        ui.set_scale_notes(ModelRc::from(Rc::new(VecModel::from(items))));
    };

    refresh();

    ui.on_update_scale(refresh);

    let ui_handle_nav = ui.as_weak();
    ui.on_change_view(move |view_name| {
        let Some(ui) = ui_handle_nav.upgrade() else {
            return;
        };
        ui.set_current_view(view_name);
    });
}
