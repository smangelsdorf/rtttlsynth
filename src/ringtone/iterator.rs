use super::*;

/// A note from the ringtone, in a form that can be played.
pub enum PlayedTone {
    /// A note with a frequency (Hz) and duration (seconds).
    Note { freq: f32, duration: f32 },

    /// A silence with a duration (seconds).
    Silence { duration: f32 },
}

/// Iterator over the tones in a ringtone, which applies the defaults and converts to the
/// `PlayedTone` type.
pub(super) struct PlayedNoteIter<'a, I>
where
    I: Iterator<Item = &'a Note>,
{
    /// The time step of a 32nd note, in seconds.
    tempo_step: f32,
    /// Iterator over the notes in the ringtone.
    notes: I,
    /// The default settings for the ringtone.
    settings: &'a Settings,
}

pub(super) fn iter<'a>(
    ringtone: &'a Ringtone,
) -> PlayedNoteIter<'a, impl Iterator<Item = &'a Note>> {
    // Tempo in quarter notes per minute, converted to a time step for a 32nd note.
    let tempo_step = 60.0 / 8.0 / ringtone.settings.tempo as f32;

    PlayedNoteIter {
        tempo_step,
        notes: ringtone.notes.iter(),
        settings: &ringtone.settings,
    }
}

impl<'a, I> Iterator for PlayedNoteIter<'a, I>
where
    I: Iterator<Item = &'a Note>,
{
    type Item = PlayedTone;

    fn next(&mut self) -> Option<Self::Item> {
        self.notes.next().map(|note| {
            // Use the default octave if the note doesn't specify one.
            let octave = note.octave.unwrap_or(self.settings.octave);

            // Use the default duration if the note doesn't specify one. Convert each note to its
            // time as a multiple of 32nd notes.
            let time = match note.duration.unwrap_or(self.settings.duration) {
                Duration::ThirtySecond => 1.0,
                Duration::Sixteenth => 2.0,
                Duration::Eighth => 4.0,
                Duration::Quarter => 8.0,
                Duration::Half => 16.0,
                Duration::Whole => 32.0,
            };

            // Dotted notes are 1.5 times as long.
            let time = if note.dotted { time * 1.5 } else { time };

            // Convert the note to a `PlayedTone`, either a note or a silence depending on whether
            // the note has a pitch.
            match note.pitch {
                None => PlayedTone::Silence {
                    duration: time * self.tempo_step,
                },
                Some(pitch) => {
                    // Convert the pitch and octave to a frequency in Hz.
                    let freq = frequency::fundamental(pitch, octave);

                    PlayedTone::Note {
                        freq,
                        duration: time * self.tempo_step,
                    }
                }
            }
        })
    }
}
