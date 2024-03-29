use std::io::Read;

use ringtone::{PlayedTone, Ringtone};
use wasm_bindgen::prelude::*;

mod ringtone;
mod sound;
mod synth;

fn err_to_string(e: Box<dyn std::error::Error>) -> String {
    e.to_string()
}

#[wasm_bindgen]
pub fn play(input: String) -> Result<(), String> {
    // Parse, and panic if the parse fails.
    let ringtone = Ringtone::parse(input.trim()).map_err(err_to_string)?;

    // Set up the output device.
    let output = sound::output();

    // WebAudio has an initial "squeak" to the sound when it starts playing instantly. 10ms of
    // silence seems to be enough to fix it.
    output.silence(0.01);

    // Play each tone in sequence.
    for tone in ringtone.iter() {
        match tone {
            PlayedTone::Note { freq, duration } => output.play(freq, duration),
            PlayedTone::Silence { duration } => output.silence(duration),
        }
    }

    // Wait for playback to complete before exiting.
    output.finish();

    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn run() {
    // The input comes from the Javascript code, which directly calls the play function.
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run() {
    // Consume all of stdin to a string.

    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("stdin read error");

    play(input).expect("play error");
}
