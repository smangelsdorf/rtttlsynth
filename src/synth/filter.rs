use rodio::Source;

// Based on https://www.w3.org/TR/audio-eq-cookbook/

pub struct BandPassFilter<S>
where
    S: Source<Item = f32>,
{
    source: S,
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,

    x_n1: f32,
    x_n2: f32,
    y_n1: f32,
    y_n2: f32,
}

impl<S> BandPassFilter<S>
where
    S: Source<Item = f32>,
{
    pub fn new(s: S, q: f32, fc: f32) -> BandPassFilter<S> {
        let fs = s.sample_rate() as f32;
        let wc = std::f32::consts::PI * 2.0 * fc / fs;

        let cosw = wc.cos();
        let sinw = wc.sin();

        let alpha = sinw / (2.0 * q);

        let a0 = 1.0 + alpha;

        let b0 = sinw / (2.0 * a0);
        let b1 = 0.0;
        let b2 = sinw / (-2.0 * a0);
        let a1 = -2.0 * cosw / a0;
        let a2 = (1.0 - alpha) / a0;

        let x_n1 = 0.0;
        let x_n2 = 0.0;
        let y_n1 = 0.0;
        let y_n2 = 0.0;

        BandPassFilter {
            source: s,
            b0,
            b1,
            b2,
            a1,
            a2,
            x_n1,
            x_n2,
            y_n1,
            y_n2,
        }
    }
}

impl<S> Iterator for BandPassFilter<S>
where
    S: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut BandPassFilter {
            b0,
            b1,
            b2,
            a1,
            a2,
            ref mut x_n1,
            ref mut x_n2,
            ref mut y_n1,
            ref mut y_n2,
            ..
        } = self;
        let x = self.source.next()?;

        let y = b0 * x + b1 * *x_n1 + b2 * *x_n2 - a1 * *y_n1 - a2 * *y_n2;

        *x_n2 = *x_n1;
        *y_n2 = *y_n1;
        *x_n1 = x;
        *y_n1 = y;

        Some(y)
    }
}

impl<S> Source for BandPassFilter<S>
where
    S: Source<Item = f32>,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.total_duration()
    }
}
