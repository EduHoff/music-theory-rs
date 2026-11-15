use super::note::Note;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    Major,
    NaturalMinor,
    HarmonicMinor,
}

impl ScaleType {
    pub const fn intervals(&self) -> &'static [u8] {
        match self {
            ScaleType::Major => &[0, 2, 4, 5, 7, 9, 11],
            ScaleType::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
            ScaleType::HarmonicMinor => &[0, 2, 3, 5, 7, 8, 11],
        }
    }

    pub const fn from_index(idx: i32) -> Self {
        match idx {
            1 => ScaleType::NaturalMinor,
            2 => ScaleType::HarmonicMinor,
            _ => ScaleType::Major,
        }
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

        Self {
            root,
            scale_type,
            notes,
        }
    }
}
