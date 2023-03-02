use super::{Octave, Pitch};

const FREQ_A4: f32 = 440.0;
const STEP: f32 = 1.059463094359;

pub(super) fn fundamental(pitch: Pitch, octave: Octave) -> f32 {
    let octave_multiplier = match octave {
        Octave::O4 => 1.0,
        Octave::O5 => 2.0,
        Octave::O6 => 4.0,
        Octave::O7 => 8.0,
    };

    let pitch_multiplier = match pitch {
        Pitch::A => 1.0,
        Pitch::Bb => STEP.powi(1),
        Pitch::B => STEP.powi(2),
        Pitch::C => STEP.powi(-9),
        Pitch::Db => STEP.powi(-8),
        Pitch::D => STEP.powi(-7),
        Pitch::Eb => STEP.powi(-6),
        Pitch::E => STEP.powi(-5),
        Pitch::F => STEP.powi(-4),
        Pitch::Gb => STEP.powi(-3),
        Pitch::G => STEP.powi(-2),
        Pitch::Ab => STEP.powi(-1),
    };

    FREQ_A4 * octave_multiplier * pitch_multiplier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fundamental() {
        fn check(freq: f32, expected: f32) {
            let diff = f32::abs(freq - expected);
            assert!(diff < 0.1, "{freq} differs from {expected}");
        }

        check(fundamental(Pitch::A, Octave::O4), 440.0);
        check(fundamental(Pitch::Ab, Octave::O5), 830.61);
        check(fundamental(Pitch::F, Octave::O4), 349.23);
        check(fundamental(Pitch::A, Octave::O6), 1760.0);
        check(fundamental(Pitch::Eb, Octave::O5), 622.25);
        check(fundamental(Pitch::C, Octave::O5), 523.25);
        check(fundamental(Pitch::B, Octave::O5), 987.77);
        check(fundamental(Pitch::E, Octave::O4), 329.63);
        check(fundamental(Pitch::Ab, Octave::O6), 1661.22);
        check(fundamental(Pitch::Db, Octave::O5), 554.37);
        check(fundamental(Pitch::Gb, Octave::O4), 369.99);
        check(fundamental(Pitch::Gb, Octave::O4), 369.99);
        check(fundamental(Pitch::Db, Octave::O6), 1108.73);
        check(fundamental(Pitch::D, Octave::O6), 1174.66);
        check(fundamental(Pitch::F, Octave::O6), 1396.91);
        check(fundamental(Pitch::B, Octave::O6), 1975.53);
        check(fundamental(Pitch::D, Octave::O7), 2349.32);
        check(fundamental(Pitch::B, Octave::O5), 987.77);
    }
}
