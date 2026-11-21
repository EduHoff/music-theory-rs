use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use std::rc::Rc;
use std::str::FromStr;

use crate::core::chord::HarmonicField;
use crate::core::note::Note;
use crate::core::scale::{Scale, ScaleType};

use crate::app::{AppWindow, ChordItem, HarmonicRowItem};

pub fn setup(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    let available_scales = ui.get_available_scales();
    let row_count = available_scales.row_count();

    let mut valid_indices = Vec::new();
    let mut valid_names = Vec::new();

    for i in 0..row_count {
        let safe_index = i32::try_from(i).unwrap_or(0);
        let scale_type = ScaleType::from_index(safe_index);

        let scale = Scale::generate(
            Note::from_str("C").expect("The note C must always be valid"),
            scale_type,
        );

        if scale.notes.len() == 7 {
            valid_indices.push(safe_index);
            if let Some(name) = available_scales.row_data(i) {
                valid_names.push(name);
            }
        }
    }

    let model = Rc::new(VecModel::from(valid_names));
    ui.set_available_harmonic_scales(ModelRc::from(model));

    let valid_indices_clone = valid_indices.clone();

    ui.on_update_harmonic_field(move || {
        let Some(ui) = ui_handle.upgrade() else {
            return;
        };

        let root_str = ui.get_selected_root().to_string();
        let is_all = root_str.starts_with("All");
        ui.set_is_all_harmonic_mode(is_all);

        let dropdown_idx = usize::try_from(ui.get_selected_harmonic_scale_idx()).unwrap_or(0);
        let real_scale_idx = valid_indices_clone.get(dropdown_idx).copied().unwrap_or(0);
        let scale_type = ScaleType::from_index(real_scale_idx);

        let build_chord_items = |root: Note| -> Vec<ChordItem> {
            let scale = Scale::generate(root, scale_type);
            HarmonicField::generate_tetrads(&scale)
                .into_iter()
                .map(|chord| ChordItem {
                    degree: SharedString::from(chord.degree_numeral),
                    chord_name: SharedString::from(format!("{}{}", chord.root, chord.suffix)),
                    notes_string: SharedString::from(
                        chord
                            .notes
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join("-"),
                    ),
                    is_active: false,
                })
                .collect()
        };

        if is_all {
            let roots_model = ui.get_available_roots();
            let mut rows = Vec::new();

            for i in 0..roots_model.row_count() {
                if let Some(r_str) = roots_model.row_data(i) {
                    let rs = r_str.to_string();
                    if rs.starts_with("All") {
                        continue;
                    }

                    if let Ok(note) = Note::from_str(&rs) {
                        let items = build_chord_items(note);
                        rows.push(HarmonicRowItem {
                            root: SharedString::from(rs),
                            chords: ModelRc::from(Rc::new(VecModel::from(items))),
                        });
                    }
                }
            }
            ui.set_all_harmonic_rows(ModelRc::from(Rc::new(VecModel::from(rows))));
        } else if let Ok(root_note) = Note::from_str(&root_str) {
            let items = build_chord_items(root_note);
            ui.set_harmonic_chords(ModelRc::from(Rc::new(VecModel::from(items))));
        }
    });

    ui.invoke_update_harmonic_field();
}
