use super::note::Note;
use super::scale::Scale;

#[derive(Debug, Clone)]
pub struct Chord {
    pub root: Note,
    pub suffix: String,
    pub notes: Vec<Note>,
    pub degree_numeral: &'static str,
}

impl Chord {
    fn determine_suffix(root: Note, third: Note, fifth: Note, seventh: Note) -> String {
        let root_semi = i32::from(root.semitone());

        let get_interval = |note: Note| -> i32 {
            let semi = i32::from(note.semitone());
            (semi - root_semi).rem_euclid(12)
        };

        let i3 = get_interval(third);
        let i5 = get_interval(fifth);
        let i7 = get_interval(seventh);

        match (i3, i5, i7) {
            (4, 7, 11) => "7M".to_string(),
            (3, 7, 10) => "m7".to_string(),
            (4, 7, 10) => "7".to_string(),
            (3, 6, 10) => "m7(b5)".to_string(),
            (3, 6, 9) => "dim".to_string(),
            (4, 8, 11) => "aug7M".to_string(),
            (4, 8, 10) => "aug7".to_string(),
            (3, 7, 11) => "m(7M)".to_string(),
            (3, 7, 9) => "m6".to_string(),
            (4, 7, 9) => "6".to_string(),
            (4, 6, 10) => "7(b5)".to_string(),
            (4, 6, 11) => "7M(b5)".to_string(),
            (3, 8, 10) => "m7(#5)".to_string(),
            _ => "?".to_string(),
        }
    }
}

pub struct HarmonicField;

impl HarmonicField {
    const DEGREES: [&'static str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];

    pub fn generate_tetrads(scale: &Scale) -> Vec<Chord> {
        let mut chords = Vec::new();
        let notes_count = scale.notes.len();

        if notes_count != 7 {
            return chords;
        }

        for i in 0..7 {
            if let (Some(&root), Some(&third), Some(&fifth), Some(&seventh)) = (
                scale.notes.get(i),
                scale.notes.get((i + 2) % 7),
                scale.notes.get((i + 4) % 7),
                scale.notes.get((i + 6) % 7),
            ) {
                let suffix = Chord::determine_suffix(root, third, fifth, seventh);

                let degree_numeral = Self::DEGREES.get(i).copied().unwrap_or("?");

                chords.push(Chord {
                    root,
                    suffix,
                    notes: vec![root, third, fifth, seventh],
                    degree_numeral,
                });
            }
        }

        chords
    }
}
