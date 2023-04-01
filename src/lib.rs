use ringtone::{PlayedTone, Ringtone};
use std::io::Read;
use wasm_bindgen::prelude::*;

mod ringtone;
mod sound;
mod synth;

#[wasm_bindgen]
pub fn play(input: String) -> Result<(), String> {
    // Parse, and panic if the parse fails.
    let ringtone = Ringtone::parse(input.trim()).map_err(|e| e.to_string())?;

    // Set up the output device.
    let output = sound::output();

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
