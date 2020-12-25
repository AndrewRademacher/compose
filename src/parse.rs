use crate::sheet::{Line, Modifier, Note, Pitch, Sheet, Value};
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{line_ending, one_of};
use nom::combinator::{map_res, opt};
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use std::str::FromStr;

pub fn sheet(input: &str) -> IResult<&str, Sheet> {
    let (input, bpm) = number_usize(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, line_value) = value(input)?;

    let (input, _) = tuple((line_ending, tag("--"), line_ending))(input)?;
    let (input, lines) = separated_list0(line_ending, line)(input)?;

    Ok((input, Sheet::new(bpm as f32, line_value, lines)))
}

pub fn line(input: &str) -> IResult<&str, Line> {
    let (input, values) = separated_list0(tag(" "), note)(input)?;
    Ok((input, Line(values)))
}

pub fn note(input: &str) -> IResult<&str, Note> {
    let (input, pitch) = pitch(input)?;
    let (input, value) = value(input)?;
    let (input, modifier) = opt(modifier)(input)?;
    let modifier = modifier.map_or(Modifier::Natural, |x| x);

    Ok((
        input,
        Note {
            pitch,
            value,
            modifier,
        },
    ))
}

fn value(input: &str) -> IResult<&str, Value> {
    let (input, indicator) = one_of("whqes")(input)?;
    let out = match indicator {
        'w' => Value::Whole,
        'h' => Value::Half,
        'q' => Value::Quarter,
        'e' => Value::Eighth,
        's' => Value::Sixteenth,
        _ => unreachable!(),
    };
    Ok((input, out))
}

fn modifier(input: &str) -> IResult<&str, Modifier> {
    let (input, indicator) = one_of("#b")(input)?;
    let out = match indicator {
        '#' => Modifier::Sharp,
        'b' => Modifier::Flat,
        _ => unreachable!(),
    };
    Ok((input, out))
}

fn pitch(input: &str) -> IResult<&str, Pitch> {
    let (input, letter) = one_of("ABCDEFG")(input)?;
    let (input, number) = take_while(is_digit)(input)?;
    Ok((input, match_pitch(letter, number)))
}

fn match_pitch(letter: char, number: &str) -> Pitch {
    match (letter, number) {
        ('A', "0") => Pitch::A0,
        ('B', "0") => Pitch::B0,
        ('C', "1") => Pitch::C1,
        ('D', "1") => Pitch::D1,
        ('E', "1") => Pitch::E1,
        ('F', "1") => Pitch::F1,
        ('G', "1") => Pitch::G1,

        ('A', "1") => Pitch::A1,
        ('B', "1") => Pitch::B1,
        ('C', "2") => Pitch::C2,
        ('D', "2") => Pitch::D2,
        ('E', "2") => Pitch::E2,
        ('F', "2") => Pitch::F2,
        ('G', "2") => Pitch::G2,

        ('A', "2") => Pitch::A2,
        ('B', "2") => Pitch::B2,
        ('C', "3") => Pitch::C3,
        ('D', "3") => Pitch::D3,
        ('E', "3") => Pitch::E3,
        ('F', "3") => Pitch::F3,
        ('G', "3") => Pitch::G3,

        ('A', "3") => Pitch::A3,
        ('B', "3") => Pitch::B3,
        ('C', "4") => Pitch::C4,
        ('D', "4") => Pitch::D4,
        ('E', "4") => Pitch::E4,
        ('F', "4") => Pitch::F4,
        ('G', "4") => Pitch::G4,

        ('A', "4") => Pitch::A4,
        ('B', "4") => Pitch::B4,
        ('C', "5") => Pitch::C5,
        ('D', "5") => Pitch::D5,
        ('E', "5") => Pitch::E5,
        ('F', "5") => Pitch::F5,
        ('G', "5") => Pitch::G5,

        ('A', "5") => Pitch::A5,
        ('B', "5") => Pitch::B5,
        ('C', "6") => Pitch::C6,
        ('D', "6") => Pitch::D6,
        ('E', "6") => Pitch::E6,
        ('F', "6") => Pitch::F6,
        ('G', "6") => Pitch::G6,

        ('A', "6") => Pitch::A6,
        ('B', "6") => Pitch::B6,
        ('C', "7") => Pitch::C7,
        ('D', "7") => Pitch::D7,
        ('E', "7") => Pitch::E7,
        ('F', "7") => Pitch::F7,
        ('G', "7") => Pitch::G7,

        ('A', "7") => Pitch::A7,
        ('B', "7") => Pitch::B7,
        ('C', "8") => Pitch::C8,

        _ => unreachable!(),
    }
}

fn number_usize(input: &str) -> IResult<&str, usize> {
    map_res(take_while(is_digit), usize::from_str)(input)
}

fn is_digit(input: char) -> bool {
    match input {
        '1' => true,
        '2' => true,
        '3' => true,
        '4' => true,
        '5' => true,
        '6' => true,
        '7' => true,
        '8' => true,
        '9' => true,
        '0' => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use crate::parse::{line, note, number_usize, sheet};
    use crate::sheet::{Line, Modifier, Note, Pitch, Sheet, Value};

    #[test]
    fn basic_sheet() {
        let input = "90xe\n--\nD3e F5h\nA4e";
        let expected = Sheet::new(
            90f32,
            Value::Eighth,
            vec![
                Line(vec![
                    Note {
                        pitch: Pitch::D3,
                        value: Value::Eighth,
                        modifier: Modifier::Natural,
                    },
                    Note {
                        pitch: Pitch::F5,
                        value: Value::Half,
                        modifier: Modifier::Natural,
                    },
                ]),
                Line(vec![Note {
                    pitch: Pitch::A4,
                    value: Value::Eighth,
                    modifier: Modifier::Natural,
                }]),
            ],
        );
        let (input, actual) = sheet(input).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(input, "");
    }

    #[test]
    fn basic_note() {
        let input = "D4q#";
        let expected = Note::new(Pitch::D4, Value::Quarter, Modifier::Sharp);
        let (_, actual) = note(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn basic_line() {
        let input = "D3e F5h";
        let expected = Line(vec![
            Note::new(Pitch::D3, Value::Eighth, Modifier::Natural),
            Note::new(Pitch::F5, Value::Half, Modifier::Natural),
        ]);
        let (_, actual) = line(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn number() {
        let input = "90f";
        let expected = 90usize;
        let (_, actual) = number_usize(input).unwrap();
        assert_eq!(actual, expected);
    }
}
