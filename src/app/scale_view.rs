use crate::app::{AppWindow, ScaleNoteItem};
use crate::core::note::Note;
use crate::core::scale::{Scale, ScaleType};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::rc::Rc;

const ROMAN_DEGREES: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];

pub fn setup(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

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
            .map(|(i, note)| ScaleNoteItem {
                degree: SharedString::from(ROMAN_DEGREES.get(i).copied().unwrap_or("-")),
                note: SharedString::from(note.to_string()),
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
