use crate::instrument::SineGenerator;
use crate::sheet::{Modifier, Note, Pitch, Sheet, Value};
use ndarray::{Array1, ArrayView1};
use rodio::Source;
use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod instrument;
mod parse;
mod sheet;
mod synth;

const SAMPLE_RATE: u32 = 96000;

fn main() {
    let mut canon_file = File::open("./sheets/canon_in_d.sht").unwrap();
    let mut canon_str = String::new();
    canon_file.read_to_string(&mut canon_str).unwrap();
    let parse_start = Instant::now();
    let (_, canon_sheet) = parse::sheet(&canon_str).unwrap();
    let parse_end = Instant::now();

    let compose_start = Instant::now();
    let generator = SineGenerator::new(40f32, SAMPLE_RATE);
    let sample = generator.compose(&canon_sheet);
    let compose_end = Instant::now();
    println!(
        "Compose in {}s",
        (compose_end - compose_start).as_secs_f32()
    );
    play_sample(sample.view());
}

fn play_sample(sample: ArrayView1<f32>) {
    let source = to_ndaudio(sample);
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    stream_handle.play_raw(source.convert_samples());

    loop {
        sleep(Duration::from_millis(10));
    }
}

pub struct NdAudio {
    data: Array1<i16>,
    pos: usize,
}

fn to_i16(input: ArrayView1<f32>) -> Array1<i16> {
    input.iter().map(|v| *v as i16).collect()
}

fn to_ndaudio(input: ArrayView1<f32>) -> NdAudio {
    NdAudio {
        data: to_i16(input),
        pos: 0,
    }
}

impl Iterator for NdAudio {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        self.pos += 1;
        match self.data.get(pos) {
            Some(v) => Some(*v),
            None => None,
        }
    }
}

impl Source for NdAudio {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs(10))
    }
}
