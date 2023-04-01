use rodio::{source::Zero, OutputStream, Sink, Source};

use crate::synth::{ADSROptions, BandPassFilter, SawWave};

pub struct OutputContext {
    sink: Sink,

    // Maintained to stop it being swept away. The `rodio` library uses `Arc` and `Weak` internally
    // to manage the lifetime of the stream, and we need to stop the stream from being dropped
    // before we're done with it.
    #[allow(dead_code)]
    stream: OutputStream,
}

impl OutputContext {
    pub fn play(&self, freq: f32, secs: f32) {
        // Fundamental wave.
        let saw = SawWave::new(freq);

        // Through trial and error, three filters were found to be the sweet spot for reproducing
        // the sound of the early Nokia phones.
        let filtered = BandPassFilter::new(saw, 1.5, 3500.0);
        let filtered = BandPassFilter::new(filtered, 1.5, 3200.0);
        let filtered = BandPassFilter::new(filtered, 1.5, 2800.0);

        // Declick the result with a short ADSR envelope.
        let source = ADSROptions::new(0.0, 0.0, 1.0, 0.005)
            .envelope(filtered, secs)
            .amplify(0.20);

        // Buffer the note to be played.
        self.sink.append(source);
    }

    pub fn silence(&self, secs: f32) {
        // All zero samples is a silent note.
        let source =
            Zero::<f32>::new(1, 48_000).take_duration(std::time::Duration::from_secs_f32(secs));

        self.sink.append(source);
    }

    pub fn finish(self) {
        // Wait for all buffered sounds to finish playing.
        self.sink.sleep_until_end()
    }
}

pub fn output() -> OutputContext {
    let (stream, stream_handle) = OutputStream::try_default().expect("opening output device");
    let sink = Sink::try_new(&stream_handle).unwrap();

    OutputContext { sink, stream }
}
