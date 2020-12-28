pub type BPM = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct Sheet {
    pub bpm: BPM,
    pub line_value: Value,
    pub lines: Vec<Line>,
}

impl Sheet {
    pub fn new(bpm: BPM, line_value: Value, lines: Vec<Line>) -> Sheet {
        Sheet {
            bpm,
            line_value,
            lines,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Line(pub Vec<Note>);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Note {
    pub pitch: Pitch,
    pub value: Value,
    pub modifier: Modifier,
}

impl Note {
    pub fn new(pitch: Pitch, value: Value, modifier: Modifier) -> Note {
        Note {
            pitch,
            value,
            modifier,
        }
    }
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Value {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
}

impl Value {
    pub fn divisor(&self) -> f32 {
        match &self {
            Value::Whole => 1.0,
            Value::Half => 0.5,
            Value::Quarter => 0.25,
            Value::Eighth => 0.125,
            Value::Sixteenth => 0.0625,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Modifier {
    Sharp,
    Natural,
    Flat,
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum Pitch {
    A0,
    B0,
    C1,
    D1,
    E1,
    F1,
    G1,

    A1,
    B1,
    C2,
    D2,
    E2,
    F2,
    G2,

    A2,
    B2,
    C3,
    D3,
    E3,
    F3,
    G3,

    A3,
    B3,
    C4,
    D4,
    E4,
    F4,
    G4,

    A4,
    B4,
    C5,
    D5,
    E5,
    F5,
    G5,

    A5,
    B5,
    C6,
    D6,
    E6,
    F6,
    G6,

    A6,
    B6,
    C7,
    D7,
    E7,
    F7,
    G7,

    A7,
    B7,
    C8,
}
