mod frequency;
mod iterator;
mod parser;

pub use iterator::PlayedTone;

pub struct Ringtone {
    #[allow(dead_code)]
    name: String,
    settings: Settings,
    notes: Vec<Note>,
}

impl Ringtone {
    pub fn parse(input: &str) -> Result<Ringtone, Box<dyn std::error::Error>> {
        parser::parse_input(input)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = PlayedTone> + 'a {
        iterator::iter(self)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Settings {
    duration: Duration,
    octave: Octave,
    tempo: Tempo,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            duration: Duration::Quarter,
            octave: Octave::O5,
            tempo: 120,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Duration {
    Whole = 1,
    Half = 2,
    Quarter = 4,
    Eighth = 8,
    Sixteenth = 16,
    ThirtySecond = 32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Octave {
    O4,
    O5,
    O6,
    O7,
}

type Tempo = u16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Note {
    duration: Option<Duration>,
    pitch: Option<Pitch>,
    octave: Option<Octave>,
    dotted: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pitch {
    A,
    // Easier than coming up with a notation for A#, etc.
    Bb,
    B,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
}
