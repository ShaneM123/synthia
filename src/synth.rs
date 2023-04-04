use fundsp::wave::Wave64;
use fundsp::hacker::*;
use anyhow::Result;

pub(crate) fn generate_basic() -> Result<()> {
let wave1 = Wave64::render(44100.0, 5.0, & mut (constant(240.0) >> sine()));
let mut filtered_wave = wave1.filter(10.0, & mut ((pass() | lfo( | t | (xerp11(110.0, 880.0, spline_noise(0, t * 5.0)), 1.0))) >> bandpass()));

// filtered_wave = filtered_wave.filter();
// let derp = Wave48::render(44100.0, 10.0, &mut (constant(240.0) >> sine())).amplitude()

    filtered_wave.normalize();
filtered_wave.save_wav16("test.wav").expect("Could not save wave.");
    Ok(())
}