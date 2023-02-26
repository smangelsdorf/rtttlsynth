pub struct Ringtone {
    pub name: String,
    pub settings: Settings,
    pub notes: Vec<Note>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Settings {
    pub duration: Duration,
    pub octave: Octave,
    pub tempo: Tempo,
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
pub enum Duration {
    Whole = 1,
    Half = 2,
    Quarter = 4,
    Eighth = 8,
    Sixteenth = 16,
    ThirtySecond = 32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Octave {
    O4,
    O5,
    O6,
    O7,
}

pub type Tempo = u16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    pub duration: Option<Duration>,
    pub pitch: Option<Pitch>,
    pub octave: Option<Octave>,
    pub dotted: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pitch {
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
