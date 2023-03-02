use ringtone::{PlayedTone, Ringtone};

mod ringtone;
mod sound;
mod synth;

pub fn run() {
    let input =
        "Nokia: d=4,o=5,b=120: 16e6, 16d6, 8f#, 8g#, 16c#6, 16b, 8d, 8e, 16b, 16a, 8c#, 8e, 2a, 2p";

    let ringtone = Ringtone::parse(input).expect("valid input");
    let output = sound::output();

    for tone in ringtone.iter() {
        match tone {
            PlayedTone::Note { freq, duration } => output.play(freq, duration),
            PlayedTone::Silence { duration } => output.silence(duration),
        }
    }

    output.finish();
}
