#[derive(Clone, Debug, PartialEq)]
pub struct Sheet {
    pub bpm: f32,
    pub line_value: Value,
    pub lines: Vec<Line>,
}

impl Sheet {
    pub fn new(bpm: f32, line_value: Value, lines: Vec<Line>) -> Sheet {
        Sheet {
            bpm,
            line_value,
            lines,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Line(pub Vec<Note>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Value {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modifier {
    Sharp,
    Natural,
    Flat,
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub enum Pitch {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,

    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,

    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,

    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,

    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,

    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,

    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,

    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,

    A9,
    B9,
    C9,
    D9,
    E9,
    F9,
    G9,

    A10,
    B10,
    C10,
    D10,
    E10,
    F10,
    G10,
}
