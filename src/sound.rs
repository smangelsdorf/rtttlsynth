use rodio::{source::Zero, OutputStream, Sink, Source};

use crate::synth::SawWave;

pub struct OutputContext {
    sink: Sink,

    // Maintained to stop it being swept away.
    #[allow(dead_code)]
    stream: OutputStream,
}

impl OutputContext {
    pub fn play(&self, freq: f32, secs: f32) {
        let source = SawWave::new(freq)
            .take_duration(std::time::Duration::from_secs_f32(secs))
            .amplify(0.20);

        self.sink.append(source);
    }

    pub fn silence(&self, secs: f32) {
        let source =
            Zero::<f32>::new(1, 48_000).take_duration(std::time::Duration::from_secs_f32(secs));

        self.sink.append(source);
    }

    pub fn finish(self) {
        self.sink.sleep_until_end()
    }
}

pub fn output() -> OutputContext {
    let (stream, stream_handle) = OutputStream::try_default().expect("opening output device");
    let sink = Sink::try_new(&stream_handle).unwrap();

    OutputContext { sink, stream }
}
