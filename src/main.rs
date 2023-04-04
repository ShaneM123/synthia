use fundsp::hacker::*;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let wave1 = Wave64::render(44100.0, 10.0, &mut (constant(240.0) >> sine()));
    let mut filtered_wave = wave1.filter(10.0, &mut ((pass() | lfo(|t| (xerp11(110.0, 880.0, spline_noise(0, t * 5.0)), 1.0))) >> bandpass()));
    filtered_wave.normalize();
    filtered_wave.save_wav16("test.wav").expect("Could not save wave.");

    // let wave3 = Wave64::load("test.wav").expect("Could not load wave.");
// Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
// Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("test.wav").unwrap());
// Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
// Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).expect("TODO: panic message");


// The sound plays in a separate audio thread,
// so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(15));
}
