mod frequency;
mod iterator;
mod parser;

pub use iterator::PlayedTone;

/// A ringtone is a sequence of notes and silences.
///
/// This is the top level structure containing the parsed ringtone.
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

/// The default settings for a ringtone, inherited by any notes that don't override them.
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

/// Note duration, relative to the tempo.
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

/// Octave of the note.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Octave {
    O4,
    O5,
    O6,
    O7,
}

/// Tempo of a ringtone.
///
/// In the original implementation this is more restrictive (i.e. the Nokia phones have a list of
/// discrete tempos that they can handle), but we'll take any u16 and do our best. YMMV at the
/// extremes.
type Tempo = u16;

/// A single note or silence.
///
/// When the duration or octave is `None`, the default value from the ringtone settings is used. If
/// the pitch is `None`, the note is a silence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Note {
    duration: Option<Duration>,
    pitch: Option<Pitch>,
    octave: Option<Octave>,
    dotted: bool,
}

/// The pitch of a note within an octave.
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
