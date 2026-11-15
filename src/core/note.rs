use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Letter {
    pub const ALL: [Letter; 7] = [
        Letter::C,
        Letter::D,
        Letter::E,
        Letter::F,
        Letter::G,
        Letter::A,
        Letter::B,
    ];

    pub const fn index(&self) -> usize {
        match self {
            Letter::C => 0,
            Letter::D => 1,
            Letter::E => 2,
            Letter::F => 3,
            Letter::G => 4,
            Letter::A => 5,
            Letter::B => 6,
        }
    }

    #[must_use]
    pub const fn next(&self, steps: usize) -> Self {
        match (self.index() + steps) % 7 {
            0 => Letter::C,
            1 => Letter::D,
            2 => Letter::E,
            3 => Letter::F,
            4 => Letter::G,
            5 => Letter::A,
            6 => Letter::B,
            _ => Letter::C,
        }
    }

    pub const fn natural_semitone(&self) -> u8 {
        match self {
            Letter::C => 0,
            Letter::D => 2,
            Letter::E => 4,
            Letter::F => 5,
            Letter::G => 7,
            Letter::A => 9,
            Letter::B => 11,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pub letter: Letter,
    pub accidental: i8,
}

impl Note {
    pub const fn new(letter: Letter, accidental: i8) -> Self {
        Self { letter, accidental }
    }

    pub fn semitone(&self) -> u8 {
        let natural = i32::from(self.letter.natural_semitone());
        let acc = i32::from(self.accidental);
        let total = (natural + acc).rem_euclid(12);

        u8::try_from(total).ok().unwrap_or_default()
    }
}

impl FromStr for Note {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let letter = match chars.next() {
            Some('C') => Letter::C,
            Some('D') => Letter::D,
            Some('E') => Letter::E,
            Some('F') => Letter::F,
            Some('G') => Letter::G,
            Some('A') => Letter::A,
            Some('B') => Letter::B,
            _ => return Err(format!("Invalid note letter in: {s}")),
        };

        let accidental = match chars.as_str() {
            "" => 0,
            "#" => 1,
            "##" => 2,
            "b" => -1,
            "bb" => -2,
            _ => return Err(format!("Invalid accidental in: {s}")),
        };

        Ok(Self { letter, accidental })
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter_str = match self.letter {
            Letter::C => "C",
            Letter::D => "D",
            Letter::E => "E",
            Letter::F => "F",
            Letter::G => "G",
            Letter::A => "A",
            Letter::B => "B",
        };
        let acc_str = match self.accidental {
            -2 => "bb",
            -1 => "b",
            0 => "",
            1 => "#",
            2 => "##",
            _ => "?",
        };
        write!(f, "{letter_str}{acc_str}")
    }
}
