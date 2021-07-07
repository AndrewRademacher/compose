use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::Result;
use ndarray::{Array1, ArrayView1};
use rodio::Source;

use crate::instrument::SineGenerator;

mod instrument;
mod parse;
mod sheet;
mod synth;

const SAMPLE_RATE: u32 = 96000;

fn main() -> Result<()> {
    let mut canon_file = File::open("./sheets/canon_in_d.sht")?;
    let mut canon_str = String::new();
    canon_file.read_to_string(&mut canon_str)?;
    let parse_timer = Instant::now();
    let (_, canon_sheet) =
        parse::sheet(&canon_str).map_err(|_| anyhow::anyhow!("Failed to parse sheet."))?;
    let parse_timer = Instant::now() - parse_timer;
    println!("Parse in {}s", parse_timer.as_secs_f32());

    let compose_timer = Instant::now();
    let mut generator = SineGenerator::new(SAMPLE_RATE);
    let sample = generator.compose(&canon_sheet);
    let compose_timer = Instant::now() - compose_timer;
    println!("Compose in {}s", compose_timer.as_secs_f32());

    let write_timer = Instant::now();
    write_sample(sample.view())?;
    let write_timer = Instant::now() - write_timer;
    println!("Written to file in {}s", write_timer.as_secs_f32());

    println!(
        "\nTotal computation time: {}s",
        (parse_timer + compose_timer + write_timer).as_secs_f32()
    );

    // play_sample(sample.view())?;

    Ok(())
}

fn write_sample(sample: ArrayView1<f32>) -> Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("canon.wav", spec)?;
    to_i16(sample)
        .iter()
        .try_for_each(|x| writer.write_sample(*x))?;
    writer.finalize()?;
    Ok(())
}

fn play_sample(sample: ArrayView1<f32>) -> Result<()> {
    let source = to_ndaudio(sample);
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
    stream_handle.play_raw(source.convert_samples())?;

    loop {
        sleep(Duration::from_millis(10));
    }
}

pub struct NdAudio {
    data: Array1<i16>,
    pos: usize,
}

fn to_i16(input: ArrayView1<f32>) -> Array1<i16> {
    // input.iter().map(|v| *v as i16).collect()
    input.iter().map(|v| (*v * 5000f32) as i16).collect()
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
