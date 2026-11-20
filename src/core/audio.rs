use crate::core::note::Note;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct ScaleStep {
    pub freq_hz: f32,
    pub duration_ms: u64,
    pub card_index: usize,
}

pub struct AudioCalculator;

impl AudioCalculator {
    pub fn note_to_hz(note_str: &str) -> f32 {
        let clean = note_str
            .split('/')
            .next()
            .unwrap_or(note_str)
            .trim()
            .to_uppercase();
        match clean.as_str() {
            "C" => 261.63,
            "C#" | "DB" => 277.18,
            "D" => 293.66,
            "D#" | "EB" => 311.13,
            "E" => 329.63,
            "F" => 349.23,
            "F#" | "GB" => 369.99,
            "G" => 392.00,
            "G#" | "AB" => 415.30,
            "A" => 440.00,
            "A#" | "BB" => 466.16,
            "B" => 493.88,
            _ => 440.00,
        }
    }

    pub fn note_to_hz_relative(root_str: &str, target_note_str: &str) -> f32 {
        let clean_root = root_str.split('/').next().unwrap_or(root_str).trim();
        let clean_target = target_note_str
            .split('/')
            .next()
            .unwrap_or(target_note_str)
            .trim();

        let base_hz = Self::note_to_hz(clean_root);

        let Ok(root_note) = Note::from_str(clean_root) else {
            return Self::note_to_hz(clean_target);
        };
        let Ok(target_note) = Note::from_str(clean_target) else {
            return base_hz;
        };

        let root_semi = i32::from(root_note.semitone());
        let target_semi = i32::from(target_note.semitone());

        let semitone_diff = (target_semi - root_semi).rem_euclid(12);
        let diff_f32 = f32::from(u8::try_from(semitone_diff).unwrap_or(0));

        base_hz * 2.0_f32.powf(diff_f32 / 12.0)
    }

    pub fn build_scale_sequence(notes: &[String]) -> Vec<ScaleStep> {
        let Some(first_note_str) = notes.first() else {
            return Vec::new();
        };

        let clean_root = first_note_str
            .split('/')
            .next()
            .unwrap_or(first_note_str)
            .trim();
        let base_hz = Self::note_to_hz(clean_root);

        let Ok(root_note) = Note::from_str(clean_root) else {
            return Vec::new();
        };
        let root_semi = i32::from(root_note.semitone());

        let mut sequence_indices = Vec::with_capacity(notes.len() * 2 + 1);
        sequence_indices.extend(0..notes.len());
        sequence_indices.push(0);
        sequence_indices.extend((0..notes.len()).rev());

        let mut steps = Vec::with_capacity(sequence_indices.len());
        for (step_idx, &note_idx) in sequence_indices.iter().enumerate() {
            if let Some(note_name) = notes.get(note_idx) {
                let clean_note = note_name.split('/').next().unwrap_or(note_name).trim();
                if let Ok(note) = Note::from_str(clean_note) {
                    let semitone = i32::from(note.semitone());
                    let semitone_diff = (semitone - root_semi).rem_euclid(12);
                    let octave_multiplier = if step_idx == notes.len() { 2.0 } else { 1.0 };
                    let diff_f32 = f32::from(u8::try_from(semitone_diff).unwrap_or(0));

                    let freq_hz = base_hz * octave_multiplier * 2.0_f32.powf(diff_f32 / 12.0);
                    let card_index = if note_idx >= notes.len() { 0 } else { note_idx };

                    steps.push(ScaleStep {
                        freq_hz,
                        duration_ms: 200,
                        card_index,
                    });
                }
            }
        }

        steps
    }
}
