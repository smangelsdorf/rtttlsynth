use super::*;

pub enum PlayedTone {
    Note { freq: f32, duration: f32 },
    Silence { duration: f32 },
}

pub(super) struct PlayedNoteIter<'a, I>
where
    I: Iterator<Item = &'a Note>,
{
    tempo_step: f32,
    notes: I,
    settings: &'a Settings,
}

pub(super) fn iter<'a>(
    ringtone: &'a Ringtone,
) -> PlayedNoteIter<'a, impl Iterator<Item = &'a Note>> {
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
            let octave = note.octave.unwrap_or(self.settings.octave);

            let time = match note.duration.unwrap_or(self.settings.duration) {
                Duration::ThirtySecond => 1.0,
                Duration::Sixteenth => 2.0,
                Duration::Eighth => 4.0,
                Duration::Quarter => 8.0,
                Duration::Half => 16.0,
                Duration::Whole => 32.0,
            };

            let time = if note.dotted { time * 1.5 } else { time };

            match note.pitch {
                None => PlayedTone::Silence {
                    duration: time * self.tempo_step,
                },
                Some(pitch) => {
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
