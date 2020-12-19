use ndarray::{Array, Array1};
use rayon::prelude::*;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::{Duration, Instant};

const SAMPLE_RATE: u32 = 96000;
// const SAMPLE_RATE: u32 = 44100;

fn main() {
    let start = Instant::now();
    play_mem();
    // play_file();
    // write_file();
    let end = Instant::now();
    println!("Total Time: {}s", (end - start).as_secs_f32());
}

fn write_file() {
    let note = build_middle_c() + build_middle_e() + build_middle_g();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("synth.wav", spec).unwrap();
    to_i16(&note)
        .iter()
        .for_each(|x| writer.write_sample(*x).unwrap());
    writer.finalize().unwrap();
}

fn play_file() {
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = File::open("canon.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples());

    loop {
        sleep(Duration::from_millis(10));
    }
}

fn play_mem() {
    let start = Instant::now();
    let c = build_middle_c();
    let e = build_middle_e();
    let g = build_middle_g();

    let note = c + e + g;
    let end = Instant::now();
    println!("Synth in {}s", (end - start).as_secs_f32());

    let source = to_ndaudio(&note);
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    stream_handle.play_raw(source.convert_samples());

    loop {
        sleep(Duration::from_millis(10));
    }
}

fn build_middle_c() -> Array1<f32> {
    let f = 261.626f32;
    build(f)
}

fn build_middle_e() -> Array1<f32> {
    let f = 329.628f32;
    build(f)
}

fn build_middle_g() -> Array1<f32> {
    let f = 391.995f32;
    build(f)
}

fn build(f: f32) -> Array1<f32> {
    let a = 5000f32;
    let pi = std::f32::consts::PI;

    let mut time = Array1::<f32>::linspace(0f32, 10f32, (SAMPLE_RATE as usize) * 10);
    time.par_map_inplace(|t| *t = a * (2f32 * pi * f * *t).sin());
    time
}

fn to_i16(input: &Array1<f32>) -> Array1<i16> {
    input.iter().map(|v| *v as i16).collect()
}

fn to_ndaudio(input: &Array1<f32>) -> NdAudio {
    NdAudio {
        data: to_i16(input),
        pos: 0,
    }
}

pub struct NdAudio {
    data: Array1<i16>,
    pos: usize,
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
