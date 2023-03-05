use rodio::Source;

#[derive(Clone, Debug)]
pub struct SawWave {
    #[allow(dead_code)]
    freq: f32,
    mult: f32,
    num_sample: usize,
}

impl SawWave {
    pub fn new(freq: f32) -> Self {
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
