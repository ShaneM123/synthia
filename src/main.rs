mod synth;
mod stream;


// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    //tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Synthia",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sine");
            if ui.button("▶").clicked() {
            synth();
            }
            else if ui.button("Rmx'd Sine").clicked() {
                remix();
            }
        });
    }
}

fn synth() {
    synth::generate_basic_sine().expect("failed to make wav");
    stream::play_stream("sine.wav".to_owned()).expect("failed to get sink");
}
fn remix() {
    synth::remix_sine().expect("failed to make wav");
    stream::play_stream("sine_rmx.wav".to_owned()).expect("failed to get sink");
}