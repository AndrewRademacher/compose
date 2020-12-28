use crate::sheet::{Modifier, Note, Pitch, Sheet, Value, BPM};
use ndarray::{s, ArcArray1, Array, Array1, ArrayView1, Zip};
use nom::bitvec::macros::internal::u8_from_ne_bits;
use nom::lib::std::collections::HashMap;

pub struct SineGenerator {
    sample_rate: u32,
    samples: HashMap<(Note, BPM), ArcArray1<f32>>,
}

impl SineGenerator {
    pub fn new(sample_rate: u32) -> SineGenerator {
        SineGenerator {
            sample_rate,
            samples: HashMap::new(),
        }
    }

    pub fn compose(&mut self, sheet: &Sheet) -> Array1<f32> {
        let line_time = 60f32 / (sheet.bpm as f32) * sheet.line_value.divisor();
        let composition_time = line_time * sheet.lines.len() as f32;
        let line_length = (line_time * self.sample_rate as f32) as usize;
        let composition_length = line_length * sheet.lines.len();

        let mut timeline = Array1::<f32>::zeros(composition_length);

        for (pos, line) in sheet.lines.iter().enumerate() {
            for note in line.0.iter() {
                let sample = self.sample(*note, sheet.bpm);
                let loc = pos * line_length;
                let loc_end = (pos * line_length) + sample.len();

                let mut view = timeline.slice_mut(s![loc..loc_end]);
                view += &sample;
            }
        }

        timeline
    }

    pub fn sample(&mut self, note: Note, bpm: BPM) -> ArcArray1<f32> {
        match self.samples.get(&(note, bpm)) {
            Some(sample) => sample.clone(),
            None => {
                let sample = self.sample_at_rate(note, bpm);
                self.samples.insert((note, bpm), sample.clone());
                sample
            }
        }
    }

    fn sample_at_rate(&self, note: Note, bpm: BPM) -> ArcArray1<f32> {
        let end_time = 60f32 / (bpm as f32) * note.value.divisor();
        let max_amplitude = 1f32;
        let pi = std::f32::consts::PI;
        let f = fundamental_frequency(&note);

        let mut time = ArcArray1::<f32>::linspace(
            0f32,
            end_time,
            ((self.sample_rate as f32 * end_time) as usize),
        );
        let percent = time.map(|t| t / end_time);
        let amplitude = percent.map(|p| match p {
            p if *p < 0.1 => (*p / 0.1) * max_amplitude,
            p if *p < 0.8 => max_amplitude,
            p => (1.0 - ((*p - 0.8) / 0.2)) * max_amplitude,
        });

        Zip::from(&mut time)
            .and(&amplitude)
            .apply(|t, &a| *t = a * (2f32 * pi * f * *t).sin());

        time
    }
}

fn fundamental_frequency(note: &Note) -> f32 {
    match (note.pitch, note.modifier) {
        (Pitch::A0, Modifier::Flat) => unreachable!(),
        (Pitch::A0, Modifier::Natural) => 27.50000,
        (Pitch::A0, Modifier::Sharp) => 29.13524,
        (Pitch::B0, Modifier::Flat) => 29.13524,
        (Pitch::B0, Modifier::Natural) => 30.86771,
        (Pitch::B0, Modifier::Sharp) => unreachable!(),
        (Pitch::C1, Modifier::Flat) => unreachable!(),
        (Pitch::C1, Modifier::Natural) => 32.70320,
        (Pitch::C1, Modifier::Sharp) => 34.64783,
        (Pitch::D1, Modifier::Flat) => 34.64783,
        (Pitch::D1, Modifier::Natural) => 36.70810,
        (Pitch::D1, Modifier::Sharp) => 38.89087,
        (Pitch::E1, Modifier::Flat) => 38.89087,
        (Pitch::E1, Modifier::Natural) => 41.20344,
        (Pitch::E1, Modifier::Sharp) => unreachable!(),
        (Pitch::F1, Modifier::Flat) => unreachable!(),
        (Pitch::F1, Modifier::Natural) => 43.65353,
        (Pitch::F1, Modifier::Sharp) => 46.24930,
        (Pitch::G1, Modifier::Flat) => 46.24930,
        (Pitch::G1, Modifier::Natural) => 48.99943,
        (Pitch::G1, Modifier::Sharp) => 51.91309,
        (Pitch::A1, Modifier::Flat) => 51.91309,
        (Pitch::A1, Modifier::Natural) => 55.00000,
        (Pitch::A1, Modifier::Sharp) => 58.27047,
        (Pitch::B1, Modifier::Flat) => 58.27047,
        (Pitch::B1, Modifier::Natural) => 61.73541,
        (Pitch::B1, Modifier::Sharp) => unreachable!(),
        (Pitch::C2, Modifier::Flat) => unreachable!(),
        (Pitch::C2, Modifier::Natural) => 65.40639,
        (Pitch::C2, Modifier::Sharp) => 69.29566,
        (Pitch::D2, Modifier::Flat) => 69.29566,
        (Pitch::D2, Modifier::Natural) => 73.41619,
        (Pitch::D2, Modifier::Sharp) => 77.78175,
        (Pitch::E2, Modifier::Flat) => 77.78175,
        (Pitch::E2, Modifier::Natural) => 82.40689,
        (Pitch::E2, Modifier::Sharp) => unreachable!(),
        (Pitch::F2, Modifier::Flat) => unreachable!(),
        (Pitch::F2, Modifier::Natural) => 87.30706,
        (Pitch::F2, Modifier::Sharp) => 92.49861,
        (Pitch::G2, Modifier::Flat) => 92.49861,
        (Pitch::G2, Modifier::Natural) => 97.99886,
        (Pitch::G2, Modifier::Sharp) => 103.8262,
        (Pitch::A2, Modifier::Flat) => 103.8262,
        (Pitch::A2, Modifier::Natural) => 110.0000,
        (Pitch::A2, Modifier::Sharp) => 116.5409,
        (Pitch::B2, Modifier::Flat) => 116.5409,
        (Pitch::B2, Modifier::Natural) => 123.4708,
        (Pitch::B2, Modifier::Sharp) => unreachable!(),
        (Pitch::C3, Modifier::Flat) => unreachable!(),
        (Pitch::C3, Modifier::Natural) => 130.8128,
        (Pitch::C3, Modifier::Sharp) => 138.5913,
        (Pitch::D3, Modifier::Flat) => 138.5913,
        (Pitch::D3, Modifier::Natural) => 146.8324,
        (Pitch::D3, Modifier::Sharp) => 155.5635,
        (Pitch::E3, Modifier::Flat) => 155.5635,
        (Pitch::E3, Modifier::Natural) => 164.8138,
        (Pitch::E3, Modifier::Sharp) => unreachable!(),
        (Pitch::F3, Modifier::Flat) => unreachable!(),
        (Pitch::F3, Modifier::Natural) => 174.6141,
        (Pitch::F3, Modifier::Sharp) => 184.9972,
        (Pitch::G3, Modifier::Flat) => 184.9972,
        (Pitch::G3, Modifier::Natural) => 195.9977,
        (Pitch::G3, Modifier::Sharp) => 207.6523,
        (Pitch::A3, Modifier::Flat) => 207.6523,
        (Pitch::A3, Modifier::Natural) => 220.0000,
        (Pitch::A3, Modifier::Sharp) => 233.0819,
        (Pitch::B3, Modifier::Flat) => 233.0819,
        (Pitch::B3, Modifier::Natural) => 246.9417,
        (Pitch::B3, Modifier::Sharp) => unreachable!(),
        (Pitch::C4, Modifier::Flat) => unreachable!(),
        (Pitch::C4, Modifier::Natural) => 261.6256,
        (Pitch::C4, Modifier::Sharp) => 277.1826,
        (Pitch::D4, Modifier::Flat) => 277.1826,
        (Pitch::D4, Modifier::Natural) => 293.6648,
        (Pitch::D4, Modifier::Sharp) => 311.1270,
        (Pitch::E4, Modifier::Flat) => 311.1270,
        (Pitch::E4, Modifier::Natural) => 329.6276,
        (Pitch::E4, Modifier::Sharp) => unreachable!(),
        (Pitch::F4, Modifier::Flat) => unreachable!(),
        (Pitch::F4, Modifier::Natural) => 349.2282,
        (Pitch::F4, Modifier::Sharp) => 369.9944,
        (Pitch::G4, Modifier::Flat) => 369.9944,
        (Pitch::G4, Modifier::Natural) => 391.9954,
        (Pitch::G4, Modifier::Sharp) => 415.3047,
        (Pitch::A4, Modifier::Flat) => 415.3047,
        (Pitch::A4, Modifier::Natural) => 440.0000,
        (Pitch::A4, Modifier::Sharp) => 466.1638,
        (Pitch::B4, Modifier::Flat) => 466.1638,
        (Pitch::B4, Modifier::Natural) => 493.8833,
        (Pitch::B4, Modifier::Sharp) => unreachable!(),
        (Pitch::C5, Modifier::Flat) => unreachable!(),
        (Pitch::C5, Modifier::Natural) => 523.2511,
        (Pitch::C5, Modifier::Sharp) => 554.3653,
        (Pitch::D5, Modifier::Flat) => 554.3653,
        (Pitch::D5, Modifier::Natural) => 587.3295,
        (Pitch::D5, Modifier::Sharp) => 622.2540,
        (Pitch::E5, Modifier::Flat) => 622.2540,
        (Pitch::E5, Modifier::Natural) => 659.2551,
        (Pitch::E5, Modifier::Sharp) => unreachable!(),
        (Pitch::F5, Modifier::Flat) => unreachable!(),
        (Pitch::F5, Modifier::Natural) => 698.4565,
        (Pitch::F5, Modifier::Sharp) => 739.9888,
        (Pitch::G5, Modifier::Flat) => 739.9888,
        (Pitch::G5, Modifier::Natural) => 783.9909,
        (Pitch::G5, Modifier::Sharp) => 830.6094,
        (Pitch::A5, Modifier::Flat) => 830.6094,
        (Pitch::A5, Modifier::Natural) => 880.0000,
        (Pitch::A5, Modifier::Sharp) => 932.3275,
        (Pitch::B5, Modifier::Flat) => 932.3275,
        (Pitch::B5, Modifier::Natural) => 987.7666,
        (Pitch::B5, Modifier::Sharp) => unreachable!(),
        (Pitch::C6, Modifier::Flat) => unreachable!(),
        (Pitch::C6, Modifier::Natural) => 1046.502,
        (Pitch::C6, Modifier::Sharp) => 1108.731,
        (Pitch::D6, Modifier::Flat) => 1108.731,
        (Pitch::D6, Modifier::Natural) => 1174.659,
        (Pitch::D6, Modifier::Sharp) => 1244.508,
        (Pitch::E6, Modifier::Flat) => 1244.508,
        (Pitch::E6, Modifier::Natural) => 1318.510,
        (Pitch::E6, Modifier::Sharp) => unreachable!(),
        (Pitch::F6, Modifier::Flat) => unreachable!(),
        (Pitch::F6, Modifier::Natural) => 1396.913,
        (Pitch::F6, Modifier::Sharp) => 1479.978,
        (Pitch::G6, Modifier::Flat) => 1479.978,
        (Pitch::G6, Modifier::Natural) => 1567.982,
        (Pitch::G6, Modifier::Sharp) => 1661.219,
        (Pitch::A6, Modifier::Flat) => 1661.219,
        (Pitch::A6, Modifier::Natural) => 1760.000,
        (Pitch::A6, Modifier::Sharp) => 1864.655,
        (Pitch::B6, Modifier::Flat) => 1864.655,
        (Pitch::B6, Modifier::Natural) => 1975.533,
        (Pitch::B6, Modifier::Sharp) => unreachable!(),
        (Pitch::C7, Modifier::Flat) => unreachable!(),
        (Pitch::C7, Modifier::Natural) => 2093.005,
        (Pitch::C7, Modifier::Sharp) => 2217.461,
        (Pitch::D7, Modifier::Flat) => 2217.461,
        (Pitch::D7, Modifier::Natural) => 2349.318,
        (Pitch::D7, Modifier::Sharp) => 2489.016,
        (Pitch::E7, Modifier::Flat) => 2489.016,
        (Pitch::E7, Modifier::Natural) => 2637.020,
        (Pitch::E7, Modifier::Sharp) => unreachable!(),
        (Pitch::F7, Modifier::Flat) => unreachable!(),
        (Pitch::F7, Modifier::Natural) => 2793.826,
        (Pitch::F7, Modifier::Sharp) => 2959.955,
        (Pitch::G7, Modifier::Flat) => 2959.955,
        (Pitch::G7, Modifier::Natural) => 3135.963,
        (Pitch::G7, Modifier::Sharp) => 3322.438,
        (Pitch::A7, Modifier::Flat) => 3322.438,
        (Pitch::A7, Modifier::Natural) => 3520.000,
        (Pitch::A7, Modifier::Sharp) => 3729.310,
        (Pitch::B7, Modifier::Flat) => 3729.310,
        (Pitch::B7, Modifier::Natural) => 3951.066,
        (Pitch::B7, Modifier::Sharp) => unreachable!(),
        (Pitch::C8, Modifier::Flat) => unreachable!(),
        (Pitch::C8, Modifier::Natural) => 4186.009,
        (Pitch::C8, Modifier::Sharp) => unreachable!(),
    }
}
