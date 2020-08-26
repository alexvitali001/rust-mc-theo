use rust_music_theory::{
    chord::{Chord, Number, Quality},
    note::{Notes, PitchClass},
};
use structopt::StructOpt;
use std::fmt;
use std::str::FromStr;

const AVAILABLE_CHORDS: [&str; 22] = [
    "Major Triad",
    "Minor Triad",
    "Suspended2 Triad",
    "Suspended4 Triad",
    "Augmented Triad",
    "Diminished Triad",
    "Major Seventh",
    "Minor Seventh",
    "Augmented Seventh",
    "Augmented Major Seventh",
    "Diminished Seventh",
    "Half Diminished Seventh",
    "Minor Major Seventh",
    "Dominant Seventh",
    "Dominant Ninth",
    "Major Ninth",
    "Dominant Eleventh",
    "Major Eleventh",
    "Minor Eleventh",
    "Dominant Thirteenth",
    "Major Thirteenth",
    "Minor Thirteenth",
];

#[derive(StructOpt, Debug)]
#[structopt(about = "Provides information for the specified chord")]
pub enum Command {
    List(ListCommand),
    Notes(NotesCommand),
}

impl Command {
    pub fn execute(self) {
        match self {
            Command::List(list_command) => list_command.execute(),
            Command::Notes(note_command) => note_command.execute(),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(alias = "l", about = "Prints out the available chords")]
pub struct ListCommand {}

impl ListCommand {
    pub fn execute(self) {
        println!("Available chords:");
        for chord in &AVAILABLE_CHORDS {
            println!(" - {}", chord);
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(alias = "n", about = "Examples:\nC minor\nAb augmented major seventh\nF# dominant seventh / C#\nC/1")]
pub struct NotesCommand {
    pitch_class: PitchClass,
    quality: Quality,
    number: Number,
    inversion: Option<Inversion>,
}

#[derive(Debug)]
enum Inversion {
    Number(u8),
    BassNote(PitchClass),
}

impl FromStr for Inversion {
    type Err = InversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse() {
            Ok(Inversion::Number(num))
        } else if let Ok(bass_note) = s.parse(){
            Ok(Inversion::BassNote(bass_note))
        } else {
            Err(InversionError)
        }
    }
}

#[derive(Debug)]
struct InversionError;

impl fmt::Display for InversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Inversion!")
    }
}

impl NotesCommand {
    pub fn execute(self) {
        let mut chord = Chord::new(self.pitch_class, self.quality, self.number);
        if let Some(inversion) = self.inversion {
            match inversion {
                Inversion::BassNote(bass_note) => {
                    if let Some(num) = chord
                        .notes()
                        .iter()
                        .position(|note| note.pitch_class == bass_note) {
                        chord.inversion = num as u8;
                    }
                }
                Inversion::Number(num) => { chord.inversion = num; }
                _ => {}
            };
        }
        chord.print_notes();
    }
}
