use crate::scale::errors::ScaleError;
use crate::scale::errors::ScaleError::ModeFromRegex;
use crate::scale::mode::Mode::*;
use regex::{Match, Regex};
use strum_macros::{Display, EnumIter};

const REGEX_MAJOR: &str = "(M|maj|Maj|Major|major|Ionian|ionian)";
const REGEX_MINOR: &str = "(m|min|Min|Minor|minor|Aeolian|aeolian)";
const REGEX_DORIAN: &str = "(dor|dorian)";
const REGEX_PHRYGIAN: &str = "(phy|phr|phrygian)";
const REGEX_LYDIAN: &str = "(lyd|lydian)";
const REGEX_MIXOLYDIAN: &str = "(mix|mixolydian)";
const REGEX_LOCRIAN: &str = "(loc|locrian)";
const REGEX_MELODIC_MINOR: &str =
    "(mel minor|melodicminor|melodic minor|Melodic Minor|MelodicMinor)";
const REGEX_HARMONIC_MINOR: &str =
    "(har minor|harmonicminor|harmonic minor|Harmonic Minor|HarmonicMinor)";

#[derive(Debug, EnumIter, Display)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    HarmonicMinor,
    MelodicMinor,
}

impl Mode {
    pub fn from_regex(string: &str) -> Result<(Self, Match), ScaleError> {
        let regexes = vec![
            (Regex::new(REGEX_MAJOR), Ionian),
            (Regex::new(REGEX_HARMONIC_MINOR), Mode::HarmonicMinor),
            (Regex::new(REGEX_MELODIC_MINOR), Mode::MelodicMinor),
            (Regex::new(REGEX_MINOR), Aeolian),
            (Regex::new(REGEX_DORIAN), Dorian),
            (Regex::new(REGEX_LOCRIAN), Locrian),
            (Regex::new(REGEX_MIXOLYDIAN), Mixolydian),
            (Regex::new(REGEX_PHRYGIAN), Phrygian),
            (Regex::new(REGEX_LYDIAN), Lydian),
        ];

        for (regex, mode_enum) in regexes {
            let mode = regex?.find(string);

            match mode {
                Some(mode_match) => return Ok((mode_enum, mode_match)),
                _ => {}
            };
        }

        Err(ModeFromRegex)
    }
}
