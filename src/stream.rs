use std::fs::File;
use std::io::BufReader;
use rodio::OutputStream;
use anyhow::Result;

pub(crate) fn play_stream(file_path: String) -> Result<()>{
let (_stream, stream_handle) = OutputStream::try_default().unwrap();
// Load a sound from a file, using a path relative to Cargo.toml
let file = BufReader::new(File::open(file_path).unwrap());

let sink = stream_handle.play_once(file).expect("could not get sink");
    sink.set_volume(0.4);
    sink.play();
    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}