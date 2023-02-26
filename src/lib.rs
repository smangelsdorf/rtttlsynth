use frequency::fundamental;
use ringtone::Duration;

mod frequency;
mod parser;
mod ringtone;
mod sound;

pub fn run() {
    let input =
        "Nokia: d=4,o=5,b=120: 16e6, 16d6, 8f#, 8g#, 16c#6, 16b, 8d, 8e, 16b, 16a, 8c#, 8e, 2a, 2p";

    let ringtone = parser::parse_input(input).expect("valid input");

    let output = sound::output();
    let tempo_step = 60.0 / 8.0 / ringtone.settings.tempo as f32;

    for note in ringtone.notes {
        let octave = note.octave.unwrap_or(ringtone.settings.octave);

        let time = match note.duration.unwrap_or(ringtone.settings.duration) {
            Duration::ThirtySecond => 1.0,
            Duration::Sixteenth => 2.0,
            Duration::Eighth => 4.0,
            Duration::Quarter => 8.0,
            Duration::Half => 16.0,
            Duration::Whole => 32.0,
        };

        let time = if note.dotted { time * 1.5 } else { time };

        match note.pitch {
            None => {}
            Some(pitch) => {
                let freq = fundamental(pitch, octave);
                output.play(freq, time * tempo_step);
            }
        }
    }

    output.finish();
}
