use rodio::Source;

/// Simple saw wave oscillator. One of the primitive waveforms.
///
/// https://en.wikipedia.org/wiki/Waveform
#[derive(Clone, Debug)]
pub struct SawWave {
    #[allow(dead_code)]
    freq: f32,
    mult: f32,
    num_sample: usize,
}

impl SawWave {
    pub fn new(freq: f32) -> Self {
        // At our fixed sample rate, we need to go through the range of [-1.0, 1.0] for each cycle
        // of the wave. So, we double it in the multiplier to go from [0.0, 1.0] to [0.0, 2.0], and
        // then subtract 1.0 below to get the range we want.
        let mult = 2.0 * freq / 48000.0;
        SawWave {
            freq,
            mult,
            num_sample: 0,
        }
    }
}

impl Iterator for SawWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.num_sample = self.num_sample.wrapping_add(1);
        // Linear increase at each step, and then using the remainder to wrap around.
        Some((self.mult * self.num_sample as f32).rem_euclid(2.0) - 1.0)
    }
}

impl Source for SawWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
