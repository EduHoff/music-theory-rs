use crate::core::audio::ScaleStep;
use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};
use slint::{Timer, TimerMode};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

struct AudioEngine {
    _sink_handle: MixerDeviceSink,
    player: Player,
}

#[derive(Clone)]
pub struct AudioPlayer {
    engine: Rc<RefCell<Option<AudioEngine>>>,
    sequence_timer: Rc<RefCell<Option<Timer>>>,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self {
            engine: Rc::new(RefCell::new(None)),
            sequence_timer: Rc::new(RefCell::new(None)),
        }
    }

    fn ensure_engine(&self) -> bool {
        let mut engine_guard = self.engine.borrow_mut();
        if engine_guard.is_none() {
            let Ok(mut sink_handle) = DeviceSinkBuilder::open_default_sink() else {
                return false;
            };
            sink_handle.log_on_drop(false);
            let player = Player::connect_new(sink_handle.mixer());

            *engine_guard = Some(AudioEngine {
                _sink_handle: sink_handle,
                player,
            });
        }
        true
    }

    fn play_raw_tone(&self, freq: f32, duration_ms: u64) {
        if !self.ensure_engine() {
            return;
        }
        if let Some(engine) = self.engine.borrow().as_ref() {
            engine.player.clear();
            let source = SineWave::new(freq)
                .take_duration(Duration::from_millis(duration_ms))
                .amplify(0.20);
            engine.player.append(source);
            engine.player.play();
        }
    }

    pub fn stop(&self) {
        if let Some(timer) = self.sequence_timer.borrow_mut().take() {
            timer.stop();
        }
        if let Some(engine) = self.engine.borrow().as_ref() {
            engine.player.clear();
            engine.player.play();
        }
    }

    pub fn play_freq(&self, freq: f32, duration_ms: u64) {
        self.stop();
        self.play_raw_tone(freq, duration_ms);
    }

    pub fn play_steps<F>(&self, steps: Vec<ScaleStep>, on_step: F)
    where
        F: Fn(Option<usize>) + 'static,
    {
        self.stop();

        let Some(first) = steps.first() else {
            return;
        };

        // Toca a primeira nota imediatamente no clique do usuário
        self.play_raw_tone(first.freq_hz, first.duration_ms);
        on_step(Some(first.card_index));

        let current_step = Rc::new(RefCell::new(1_usize));
        let total_steps = steps.len();
        let player_clone = self.clone();
        let timer_holder = Rc::clone(&self.sequence_timer);
        let timer = Timer::default();

        timer.start(TimerMode::Repeated, Duration::from_millis(220), move || {
            let mut idx = current_step.borrow_mut();
            if *idx < total_steps {
                if let Some(step) = steps.get(*idx) {
                    player_clone.play_raw_tone(step.freq_hz, step.duration_ms);
                    on_step(Some(step.card_index));
                }
                *idx += 1;
            } else {
                on_step(None);
                if let Some(t) = timer_holder.borrow_mut().take() {
                    t.stop();
                }
            }
        });

        *self.sequence_timer.borrow_mut() = Some(timer);
    }
}
