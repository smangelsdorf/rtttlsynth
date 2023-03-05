use std::{iter::Sum, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{digit1, multispace0, one_of},
    combinator::{eof, map, map_parser, map_res, opt, recognize, value},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult, Parser,
};

use super::*;

fn base10_numeric<N>(input: &str) -> IResult<&str, N>
where
    N: Sum<N> + FromStr,
{
    map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        N::from_str(s)
    })
    .parse(input)
}

fn section_separator(input: &str) -> IResult<&str, ()> {
    value((), delimited(multispace0, tag(":"), multispace0)).parse(input)
}

fn item_separator(input: &str) -> IResult<&str, ()> {
    value((), delimited(multispace0, tag(","), multispace0)).parse(input)
}

fn name(input: &str) -> IResult<&str, &str> {
    take_till1(|c| c == ':').parse(input)
}

fn duration(input: &str) -> IResult<&str, Duration> {
    map_parser(
        digit1,
        alt((
            value(Duration::ThirtySecond, tag("32")),
            value(Duration::Sixteenth, tag("16")),
            value(Duration::Eighth, tag("8")),
            value(Duration::Quarter, tag("4")),
            value(Duration::Half, tag("2")),
            value(Duration::Whole, tag("1")),
        )),
    )
    .parse(input)
}

fn octave(input: &str) -> IResult<&str, Octave> {
    map_parser(
        digit1,
        alt((
            value(Octave::O4, tag("4")),
            value(Octave::O5, tag("5")),
            value(Octave::O6, tag("6")),
            value(Octave::O7, tag("7")),
        )),
    )
    .parse(input)
}

fn tempo(input: &str) -> IResult<&str, u16> {
    base10_numeric.parse(input)
}

#[derive(Clone, Copy)]
enum Setting {
    Duration(Duration),
    Octave(Octave),
    Tempo(Tempo),
    Ignored,
}

impl Setting {
    fn put(settings: Settings, setting: Setting) -> Settings {
        match setting {
            Setting::Duration(duration) => Settings {
                duration,
                ..settings
            },
            Setting::Octave(octave) => Settings { octave, ..settings },
            Setting::Tempo(tempo) => Settings { tempo, ..settings },
            Setting::Ignored => settings,
        }
    }
}

fn setting(input: &str) -> IResult<&str, Setting> {
    alt((
        map(preceded(tag("d="), duration), Setting::Duration),
        map(preceded(tag("o="), octave), Setting::Octave),
        map(preceded(tag("b="), tempo), Setting::Tempo),
        value(Setting::Ignored, preceded(tag("l="), base10_numeric::<u32>)),
        value(Setting::Ignored, preceded(tag("s="), base10_numeric::<u32>)),
    ))
    .parse(input)
}

fn settings(input: &str) -> IResult<&str, Settings> {
    map(separated_list1(item_separator, setting), |list| {
        list.into_iter().fold(Settings::default(), Setting::put)
    })
    .parse(input)
}

fn pitch(input: &str) -> IResult<&str, Option<Pitch>> {
    map_res(
        tuple((one_of("abcdefgp"), opt(tag("#")))),
        |pitch| match pitch {
            ('a', None) => Ok(Some(Pitch::A)),
            ('a', Some(_)) => Ok(Some(Pitch::Bb)),
            ('b', None) => Ok(Some(Pitch::B)),
            ('c', None) => Ok(Some(Pitch::C)),
            ('c', Some(_)) => Ok(Some(Pitch::Db)),
            ('d', None) => Ok(Some(Pitch::D)),
            ('d', Some(_)) => Ok(Some(Pitch::Eb)),
            ('e', None) => Ok(Some(Pitch::E)),
            ('f', None) => Ok(Some(Pitch::F)),
            ('f', Some(_)) => Ok(Some(Pitch::Gb)),
            ('g', None) => Ok(Some(Pitch::G)),
            ('g', Some(_)) => Ok(Some(Pitch::Ab)),
            ('p', None) => Ok(None),
            _ => Err("No such pitch"),
        },
    )
    .parse(input)
}

fn note(input: &str) -> IResult<&str, Note> {
    map(
        tuple((
            opt(duration),
            pitch,
            opt(octave),
            map(opt(tag(".")), |o| o.is_some()),
        )),
        |(duration, pitch, octave, dotted)| Note {
            duration,
            pitch,
            octave,
            dotted,
        },
    )
    .parse(input)
}

fn notes(input: &str) -> IResult<&str, Vec<Note>> {
    separated_list1(item_separator, note).parse(input)
}

fn ringtone(input: &str) -> IResult<&str, Ringtone> {
    map(
        tuple((
            map(terminated(name, section_separator), |s| s.to_owned()),
            terminated(settings, section_separator),
            terminated(notes, eof),
        )),
        |(name, settings, notes)| Ringtone {
            name,
            settings,
            notes,
        },
    )
    .parse(input)
}

pub(super) fn parse_input(input: &str) -> Result<Ringtone, Box<dyn std::error::Error>> {
    let (_rest, ringtone) = ringtone.parse(input).map_err(|e| e.to_owned()).finish()?;
    Ok(ringtone)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name() {
        let input = "Ringtone! Hi!:";

        assert_eq!(name(input), Ok((":", "Ringtone! Hi!")));
    }

    #[test]
    fn test_parse_settings() {
        let input = "d=4,o=7,b=108";

        assert_eq!(
            settings(input),
            Ok((
                "",
                Settings {
                    duration: Duration::Quarter,
                    octave: Octave::O7,
                    tempo: 108,
                }
            ))
        );

        let input = "d=16";

        assert_eq!(
            settings(input),
            Ok((
                "",
                Settings {
                    duration: Duration::Sixteenth,
                    octave: Octave::O5,
                    tempo: 120,
                }
            ))
        );

        let input = "o=4";

        assert_eq!(
            settings(input),
            Ok((
                "",
                Settings {
                    duration: Duration::Quarter,
                    octave: Octave::O4,
                    tempo: 120,
                }
            ))
        );
    }

    #[test]
    fn test_parse_note() {
        assert_eq!(
            note("2a4"),
            Ok((
                "",
                Note {
                    duration: Some(Duration::Half),
                    pitch: Some(Pitch::A),
                    octave: Some(Octave::O4),
                    dotted: false,
                },
            ))
        );

        assert_eq!(
            note("2e"),
            Ok((
                "",
                Note {
                    duration: Some(Duration::Half),
                    pitch: Some(Pitch::E),
                    octave: None,
                    dotted: false,
                },
            ))
        );

        assert_eq!(
            note("2d#"),
            Ok((
                "",
                Note {
                    duration: Some(Duration::Half),
                    pitch: Some(Pitch::Eb),
                    octave: None,
                    dotted: false,
                },
            ))
        );

        assert_eq!(
            note("32a#4"),
            Ok((
                "",
                Note {
                    duration: Some(Duration::ThirtySecond),
                    pitch: Some(Pitch::Bb),
                    octave: Some(Octave::O4),
                    dotted: false,
                },
            ))
        );

        assert_eq!(
            note("8c#7."),
            Ok((
                "",
                Note {
                    duration: Some(Duration::Eighth),
                    pitch: Some(Pitch::Db),
                    octave: Some(Octave::O7),
                    dotted: true,
                },
            ))
        );

        assert_eq!(
            note("g"),
            Ok((
                "",
                Note {
                    duration: None,
                    pitch: Some(Pitch::G),
                    octave: None,
                    dotted: false,
                },
            ))
        );

        assert_eq!(
            note("4p."),
            Ok((
                "",
                Note {
                    duration: Some(Duration::Quarter),
                    pitch: None,
                    octave: None,
                    dotted: true,
                },
            ))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "Nokia: d=4,o=5,b=120,l=5,s=4: 16e6, 16d6, 8f#, 8g#, 16c#6, 16b, 8d, 8e, 16b, 16a, 8c#, 8e, 2a, 2p";

        let ringtone = parse_input(input).expect("successful parse");

        assert_eq!(ringtone.name, "Nokia");
        assert_eq!(
            ringtone.settings,
            Settings {
                duration: Duration::Quarter,
                octave: Octave::O5,
                tempo: 120
            }
        );

        assert_eq!(ringtone.notes.len(), 14);
    }
}
