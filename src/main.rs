use rustfft::{Fft, FftPlanner, FftDirection, num_complex::Complex, algorithm::Radix4};
use std::io;

mod fuzz;
mod utils;

mod audio_server;
use audio_server::{audio_sink::{AudioSink, SinkType}, jack_audio_sink::{self, JackAudioSink}};

use fuzz::FuzzFace;
use utils::critical_error_handler;

fn main() {
    let name = "123";
    let mut sink = JackAudioSink::new(name, SinkType::OneToOne).unwrap();
    sink.init(2);
    sink.xxx();

    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();
}