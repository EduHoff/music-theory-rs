use super::note::Note;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    Chromatic,
    NaturalMajor,
    NaturalMinor,
    HarmonicMinor,
    HarmonicMajor,
    MelodicMinor,
    MelodicMajor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    WholeTone,
    Arabic,
    Diminished,
    MajorPentatonic,
    MinorPentatonic,
    Blues,
    Hirajoshi,
}

impl ScaleType {
    pub const ALL: &[ScaleType] = &[
        ScaleType::Chromatic,
        ScaleType::NaturalMajor,
        ScaleType::NaturalMinor,
        ScaleType::HarmonicMinor,
        ScaleType::HarmonicMajor,
        ScaleType::MelodicMinor,
        ScaleType::MelodicMajor,
        ScaleType::Dorian,
        ScaleType::Phrygian,
        ScaleType::Lydian,
        ScaleType::Mixolydian,
        ScaleType::Locrian,
        ScaleType::WholeTone,
        ScaleType::Arabic,
        ScaleType::Diminished,
        ScaleType::MajorPentatonic,
        ScaleType::MinorPentatonic,
        ScaleType::Blues,
        ScaleType::Hirajoshi,
    ];

    pub const fn name(&self) -> &'static str {
        match self {
            ScaleType::Chromatic => "Chromatic",
            ScaleType::NaturalMajor => "Natural Major / Ionian",
            ScaleType::NaturalMinor => "Natural Minor / Aeolian",
            ScaleType::HarmonicMinor => "Harmonic Minor",
            ScaleType::HarmonicMajor => "Harmonic Major",
            ScaleType::MelodicMinor => "Melodic Minor",
            ScaleType::MelodicMajor => "Melodic Major",
            ScaleType::Dorian => "Dorian",
            ScaleType::Phrygian => "Phrygian",
            ScaleType::Lydian => "Lydian",
            ScaleType::Mixolydian => "Mixolydian",
            ScaleType::Locrian => "Locrian",
            ScaleType::WholeTone => "Whole Tone",
            ScaleType::Arabic => "Arabic (Double Harmonic)",
            ScaleType::Diminished => "Diminished (H-W)",
            ScaleType::MajorPentatonic => "Major Pentatonic",
            ScaleType::MinorPentatonic => "Minor Pentatonic",
            ScaleType::Blues => "Blues Scale",
            ScaleType::Hirajoshi => "Hirajoshi",
        }
    }

    pub const fn intervals(&self) -> &'static [u8] {
        match self {
            ScaleType::Chromatic => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            ScaleType::NaturalMajor => &[0, 2, 4, 5, 7, 9, 11],
            ScaleType::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
            ScaleType::HarmonicMinor => &[0, 2, 3, 5, 7, 8, 11],
            ScaleType::HarmonicMajor => &[0, 2, 4, 5, 7, 8, 11],
            ScaleType::MelodicMinor => &[0, 2, 3, 5, 7, 9, 11],
            ScaleType::MelodicMajor => &[0, 2, 4, 5, 7, 8, 11],
            ScaleType::Dorian => &[0, 2, 3, 5, 7, 9, 10],
            ScaleType::Phrygian => &[0, 1, 3, 5, 7, 8, 10],
            ScaleType::Lydian => &[0, 2, 4, 6, 7, 9, 11],
            ScaleType::Mixolydian => &[0, 2, 4, 5, 7, 9, 10],
            ScaleType::Locrian => &[0, 1, 3, 5, 6, 8, 10],
            ScaleType::WholeTone => &[0, 2, 4, 6, 8, 10],
            ScaleType::Arabic => &[0, 1, 4, 5, 7, 8, 11],
            ScaleType::Diminished => &[0, 2, 3, 5, 6, 8, 9, 11],
            ScaleType::MajorPentatonic => &[0, 2, 4, 7, 9],
            ScaleType::MinorPentatonic => &[0, 3, 5, 7, 10],
            ScaleType::Blues => &[0, 3, 5, 6, 7, 10],
            ScaleType::Hirajoshi => &[0, 2, 3, 7, 8],
        }
    }

    pub fn from_index(idx: i32) -> Self {
        let Ok(usize_idx) = usize::try_from(idx) else {
            return ScaleType::Chromatic;
        };
        *Self::ALL.get(usize_idx).unwrap_or(&ScaleType::Chromatic)
    }
}

pub struct Scale {
    pub root: Note,
    pub scale_type: ScaleType,
    pub notes: Vec<Note>,
}

impl Scale {
    pub fn generate(root: Note, scale_type: ScaleType) -> Self {
        let root_semi = i32::from(root.semitone());
        let intervals = scale_type.intervals();
        let mut notes = Vec::with_capacity(intervals.len());

        if scale_type == ScaleType::Chromatic {
            for &interval in intervals {
                let absolute_semi = (root_semi + i32::from(interval)).rem_euclid(12);

                let (letter, acc) = match absolute_semi {
                    0 => (super::note::Letter::C, 0),
                    1 => (super::note::Letter::C, 1),
                    2 => (super::note::Letter::D, 0),
                    3 => (super::note::Letter::D, 1),
                    4 => (super::note::Letter::E, 0),
                    5 => (super::note::Letter::F, 0),
                    6 => (super::note::Letter::F, 1),
                    7 => (super::note::Letter::G, 0),
                    8 => (super::note::Letter::G, 1),
                    9 => (super::note::Letter::A, 0),
                    10 => (super::note::Letter::A, 1),
                    11 => (super::note::Letter::B, 0),
                    _ => (super::note::Letter::C, 0),
                };

                notes.push(Note::new(letter, acc));
            }
        } else {
            for (step, &interval) in intervals.iter().enumerate() {
                let target_letter = root.letter.next(step);

                let interval_i32 = i32::from(interval);
                let target_semi = (root_semi + interval_i32).rem_euclid(12);
                let natural_semi = i32::from(target_letter.natural_semitone());

                let mut diff = target_semi - natural_semi;
                if diff > 6 {
                    diff -= 12;
                } else if diff < -6 {
                    diff += 12;
                }

                let accidental = i8::try_from(diff).unwrap_or_default();
                notes.push(Note::new(target_letter, accidental));
            }
        }

        Self {
            root,
            scale_type,
            notes,
        }
    }
}
