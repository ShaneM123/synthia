use fundsp::wave::Wave64;
use fundsp::hacker::*;
use anyhow::Result;
use hound;

pub(crate) fn generate_basic_sine() -> Result<()> {
let wave1 = Wave64::render(44100.0, 5.0, & mut (constant(240.0) >> sine()));
let mut filtered_wave = wave1.filter(10.0, & mut ((pass() | lfo( | t | (xerp11(110.0, 880.0, spline_noise(0, t * 5.0)), 1.0))) >> bandpass()));

filtered_wave.normalize();
filtered_wave.save_wav32("sine.wav").expect("Could not save wave.");
    Ok(())
}

pub(crate) fn remix_sine() -> Result<()> {
   let mut reader = hound::WavReader::open("sine.wav").unwrap();
   let sample_rate = reader.spec().sample_rate;
   let samples = reader.samples::<f32>().map(|x| x.unwrap()).collect::<Vec<f32>>();
   
    let wave1 = Wave32::from_samples(sample_rate as f64, &samples);
    let mut filtered_wave = wave1.filter(10.0, & mut ((hacker32::pass() | hacker32::lfo( | t | (hacker32::xerp11(220.0, 990.0, hacker32::spline_noise(0, t * 24.0)), 1.0))) >> hacker32::bandpass()));
    filtered_wave.normalize();
    filtered_wave.save_wav16("sine_rmx.wav").expect("Could not save wave.");
        Ok(())
    }
