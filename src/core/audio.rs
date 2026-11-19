use crate::core::note::Note;
use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, Player};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AudioManager {
    current_player: Arc<Mutex<Option<Player>>>,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            current_player: Arc::new(Mutex::new(None)),
        })
    }

    pub fn stop(&self) {
        if let Ok(mut player_guard) = self.current_player.lock()
            && let Some(player) = player_guard.take()
        {
            drop(player);
        }
    }

    pub fn play_freq(&self, freq: f32, duration_ms: u64) {
        self.stop();

        let player_arc = Arc::clone(&self.current_player);

        std::thread::spawn(move || {
            let Ok(mut sink_handle) = DeviceSinkBuilder::open_default_sink() else {
                return;
            };
            sink_handle.log_on_drop(false);

            let player = Player::connect_new(sink_handle.mixer());

            let source = SineWave::new(freq)
                .take_duration(Duration::from_millis(duration_ms))
                .amplify(0.20);

            player.append(source);

            if let Ok(mut player_guard) = player_arc.lock() {
                *player_guard = Some(player);
            }

            std::thread::sleep(Duration::from_millis(duration_ms + 50));
        });
    }

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

    pub fn play_scale_sequence<F>(&self, notes: Vec<String>, on_note_play: F)
    where
        F: Fn(Option<usize>) + Send + Sync + 'static,
    {
        self.stop();

        let callback = Arc::new(on_note_play);

        std::thread::spawn(move || {
            let Ok(mut sink_handle) = DeviceSinkBuilder::open_default_sink() else {
                return;
            };
            sink_handle.log_on_drop(false);

            let player = Player::connect_new(sink_handle.mixer());

            let Some(first_note_str) = notes.first() else {
                return;
            };

            let clean_root = first_note_str
                .split('/')
                .next()
                .unwrap_or(first_note_str)
                .trim();
            let base_hz = Self::note_to_hz(clean_root);

            let Ok(root_note) = Note::from_str(clean_root) else {
                return;
            };

            let root_semi = i32::from(root_note.semitone());

            let mut sequence_indices = Vec::new();
            for i in 0..notes.len() {
                sequence_indices.push(i);
            }
            sequence_indices.push(0);
            for i in (0..notes.len()).rev() {
                sequence_indices.push(i);
            }

            let mut full_sequence = Vec::with_capacity(sequence_indices.len());
            for &idx in &sequence_indices {
                if let Some(note_str) = notes.get(idx) {
                    full_sequence.push(note_str.clone());
                }
            }

            for (step_idx, &note_idx) in sequence_indices.iter().enumerate() {
                let Some(note_name) = full_sequence.get(step_idx) else {
                    continue;
                };
                let clean_note = note_name.split('/').next().unwrap_or(note_name).trim();

                if let Ok(note) = Note::from_str(clean_note) {
                    let semitone = i32::from(note.semitone());
                    let semitone_diff = (semitone - root_semi).rem_euclid(12);

                    let octave_multiplier = if step_idx == notes.len() { 2.0 } else { 1.0 };
                    let diff_f32 = f32::from(u8::try_from(semitone_diff).unwrap_or(0));

                    let freq = base_hz * octave_multiplier * 2.0_f32.powf(diff_f32 / 12.0);

                    let source = SineWave::new(freq)
                        .take_duration(Duration::from_millis(200))
                        .amplify(0.20);

                    player.append(source);

                    let target_card_idx = if note_idx >= notes.len() { 0 } else { note_idx };
                    let cb_clone = Arc::clone(&callback);
                    let _ = slint::invoke_from_event_loop(move || {
                        cb_clone(Some(target_card_idx));
                    });

                    std::thread::sleep(Duration::from_millis(220));
                }
            }

            let cb_clone = Arc::clone(&callback);
            let _ = slint::invoke_from_event_loop(move || {
                cb_clone(None);
            });

            player.sleep_until_end();
        });
    }
}
