# rtttlsynth

A simple [RTTTL (RingTone Text Transfer Language)][rtttl] synthesizer written in Rust.

I wrote this to learn more about synthesizers and audio programming in general. The synthesizer is
based on Rust's `rodio` library, with the oscillator and filters implemented from scratch.

[rtttl]: https://en.wikipedia.org/wiki/Ring_Tone_Text_Transfer_Language

## Usage

For convenience, this also runs as WASM and is available at
<https://smangelsdorf.github.io/rtttlsynth/>.

To run the synthesizer locally, you need to have the Rust environment installed. Then, run

    cargo run < examples/nokia.txt

## License

This project is licensed under the terms of the [MIT license](https://opensource.org/licenses/MIT).