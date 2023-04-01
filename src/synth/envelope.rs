use std::time::Duration;

use rodio::Source;

/// Options for an ADSR envelope.
///
/// https://en.wikipedia.org/wiki/Envelope_(music)
//
// Confession time, I only added this to remove the tiny click when a note turns off. Full linear
// ADSR implementation to make a 5ms fade-out on each note. Total overkill.
pub struct ADSROptions {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl ADSROptions {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        ADSROptions {
            attack,
            decay,
            sustain,
            release,
        }
    }

    pub fn envelope<S: Source + Iterator<Item = f32>>(
        &self,
        source: S,
        note_duration: f32,
    ) -> ADSR<S> {
        let sample_rate = source.sample_rate() as f32;
        // Calculate the actual number of samples for each stage of the envelope.
        let attack = (self.attack * sample_rate) as u32;
        let decay = (self.decay * sample_rate) as u32;
        let release = (self.release * sample_rate) as u32;

        // Contrary to most ADSR envelope implementations, the release is considered part of the
        // note duration. This makes it easier to handle because ordinarily the envelope is a
        // function based on the input, but in our case our input is the total duration of the note.
        let sustain_duration = ((note_duration * sample_rate) as u32)
            .checked_sub(attack + decay + release)
            .unwrap_or(0);

        let sustain = self.sustain;

        ADSR {
            source,
            index: 0,
            attack_end: attack,
            decay_end: attack + decay,
            sustain,
            sustain_end: attack + decay + sustain_duration,
            release_end: attack + decay + sustain_duration + release,
        }
    }
}

/// An ADSR envelope, bound to a source note.
pub struct ADSR<S: Source + Iterator<Item = f32>> {
    source: S,
    index: u32,
    attack_end: u32,
    decay_end: u32,
    sustain: f32,
    sustain_end: u32,
    release_end: u32,
}

impl<S> Iterator for ADSR<S>
where
    S: Source + Iterator<Item = f32>,
{
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let &mut ADSR {
            ref mut source,
            ref mut index,
            attack_end,
            decay_end,
            sustain,
            sustain_end,
            release_end,
            ..
        } = self;

        let sample = source.next()?;
        *index += 1;

        let mult = if *index < attack_end {
            // Linear increase from 0.0 to 1.0 over the attack duration.
            *index as f32 / attack_end as f32
        } else if *index < decay_end {
            // Linear decrease from 1.0 to the sustain level over the decay duration.
            let decay_duration = (decay_end - attack_end) as f32;
            let decay_elapsed = (*index - attack_end) as f32;
            sustain + (1.0 - sustain) * (1.0 - decay_elapsed / decay_duration)
        } else if *index < sustain_end {
            // Constant sustain level.
            sustain
        } else if *index < release_end {
            // Linear decrease from the sustain level to 0.0 over the release duration.
            let release_duration = release_end - sustain_end;
            sustain * (1.0 - (*index - sustain_end) as f32 / release_duration as f32)
        } else {
            // Past the end of the envelope.
            return None;
        };

        Some(mult * sample)
    }
}

impl<S> Source for ADSR<S>
where
    S: Source + Iterator<Item = f32>,
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        // The lowest of the current frame length of the source and the remaining samples in the
        // envelope.
        let remaining = (self.release_end - self.index) as usize;

        Some(
            self.source
                .current_frame_len()
                .map_or(remaining, |len| len.min(remaining)),
        )
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        let total = Duration::from_secs_f32(self.release_end as f32 / self.sample_rate() as f32);

        Some(self.source.total_duration().map_or(total, |d| d.min(total)))
    }
}
