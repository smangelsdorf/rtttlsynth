use ringtone::{PlayedTone, Ringtone};
use wasm_bindgen::prelude::*;

mod ringtone;
mod sound;
mod synth;

#[wasm_bindgen]
pub fn play(input: String) -> Result<(), String> {
    let ringtone = Ringtone::parse(input.trim()).expect("valid input");
    let output = sound::output();

    for tone in ringtone.iter() {
        match tone {
            PlayedTone::Note { freq, duration } => output.play(freq, duration),
            PlayedTone::Silence { duration } => output.silence(duration),
        }
    }

    output.finish();

    Ok(())
}

pub fn run() {
    let input = r"
        Badinerie:d=16,o=5,b=210,l=15:
        a.,8p,c6.,a.,e.,8p,a.,e.,c.,p.,e.,c.,4a4,8p,
        e4.,a4.,c.,a4.,b4.,a4.,b4.,a4.,g#4.,b4.,d.,b4.,8c.,8a4.,
        a.,8p,c6.,a.,e.,p.,a.,e.,c.,8p,e.,c.,4a4,8p,
        c.,b4.,8c,p,c.,b4.,8c.,8a.,c,p.,32d,8c.,b4,8p,
        e.,d#.,8e,p,e.,d#.,8e.,8c6.,e,p.,32f#,8e.,d#,8p,
        b4.,e.,g.,e.,f#.,e.,f#.,e.,d#.,f#.,a.,f#.,g.,f#.,e.,d#.,
        e.,g.,e.,d#.,e.,a.,e.,d#.,e.,b.,e.,d#.,e.,c6.,e.,d#.,e.,
        c6.,b.,a.,b.,g.,f#.,e.,8g,32f#,32g,8f#,e,4e,8p,

        e.,8p,g.,e.,b4.,8p,e.,b4.,g4.,p.,b4.,g4.,4e4,8p,
        8a#4.,8a4.,8d.,8c#,e.,8g.,f.,e.,8f.,8d.,32p,
        f.,8p,a.,f.,d.,8p,f.,d.,b4.,p.,d.,b4.,4g4,8p.,
        c.,e.,c.,d.,c.,d.,c.,b4.,d.,f.,d.,e.,d.,c.,b4.,
        c.,e.,c.,b4.,c.,f.,c.,b4.,c.,g.,c.,b4.,c.,a.,c.,b4.,c.,
        a.,g.,f.,g.,e.,d.,c.,8e.,32d,32e,d.,c,4c,8p,

        e.,d.,8e,p,e.,d.,8e.,8c6.,e,p.,32f,8e.,d,8p,
        d.,c#.,8d,p,d.,c#.,8d.,8b.,d,p.,32e,8d.,c,8p,
        a.,8p,32c6.,32b.,32a.,32g.,2f,8p,32a.,32g.,32f.,32e.,2d,p,
        32f.,32e.,32d.,32c.,a#4.,d.,f.,d.,a#4.,a4.,a#4.,a4.,
        8g#4.,8e4.,8f4.,e4.,8b4,8d.,c.,b4.,8c,p,a4,b4,c,d,e,p,
        c.,e.,8a,p,8e,p,d,32p,c,32p,b4,32p,c,32p,2a4";

    let ringtone = Ringtone::parse(input.trim()).expect("valid input");
    let output = sound::output();

    for tone in ringtone.iter() {
        match tone {
            PlayedTone::Note { freq, duration } => output.play(freq, duration),
            PlayedTone::Silence { duration } => output.silence(duration),
        }
    }

    output.finish();
}
