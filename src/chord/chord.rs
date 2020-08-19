use crate::chord::errors::ChordError;
use crate::chord::number::Number::Triad;
use crate::chord::{Number, Quality};
use crate::interval::Interval;
use crate::note::{Note, NoteError, Notes, PitchClass};

/// A chord.
#[derive(Debug, Clone)]
pub struct Chord {
    /// The root note of the chord.
    pub root: PitchClass,
    /// The octave of the root note of the chord.
    pub octave: u8,
    /// The intervals within the chord.
    pub intervals: Vec<Interval>,
    /// The quality of the chord: major, minor, diminished, etc.
    pub quality: Quality,
    /// The superscript number of the chord: 3, 7, maj7, etc.
    pub number: Number,
    /// The inversion of the chord: 0=root position, 1=first inversion, etc.
    pub inversion: u8,
}

impl Chord {
    /// Create a new chord.
    pub fn new(root: PitchClass, quality: Quality, number: Number) -> Self {
        Self::with_inversion(root, quality, number, 0)
    }

    /// Create a new chord with a given inversion.
    pub fn with_inversion(
        root: PitchClass,
        quality: Quality,
        number: Number,
        inversion: u8,
    ) -> Self {
        let intervals = Self::chord_intervals(quality, number);
        let inversion = inversion % (intervals.len() + 1) as u8;
        Chord {
            root,
            octave: 4,
            intervals,
            quality,
            number,
            inversion,
        }
    }

    pub fn chord_intervals(quality: Quality, number: Number) -> Vec<Interval> {
        use Number::*;
        use Quality::*;
        match (&quality, &number) {
            (Major, Triad) => Interval::from_semitones(&[4, 3]),
            (Minor, Triad) => Interval::from_semitones(&[3, 4]),
            (Suspended2, Triad) => Interval::from_semitones(&[2, 5]),
            (Suspended4, Triad) => Interval::from_semitones(&[5, 7]),
            (Augmented, Triad) => Interval::from_semitones(&[4, 4]),
            (Diminished, Triad) => Interval::from_semitones(&[3, 3]),
            (Major, Seventh) => Interval::from_semitones(&[4, 3, 4]),
            (Minor, Seventh) => Interval::from_semitones(&[3, 4, 3]),
            (Augmented, Seventh) => Interval::from_semitones(&[4, 4, 2]),
            (Augmented, MajorSeventh) => Interval::from_semitones(&[4, 4, 3]),
            (Diminished, Seventh) => Interval::from_semitones(&[3, 3, 3]),
            (HalfDiminished, Seventh) => Interval::from_semitones(&[3, 3, 4]),
            (Minor, MajorSeventh) => Interval::from_semitones(&[3, 4, 4]),
            (Dominant, Seventh) => Interval::from_semitones(&[4, 3, 3]),
            (Dominant, Ninth) => Interval::from_semitones(&[4, 3, 3, 4]),
            (Major, Ninth) => Interval::from_semitones(&[4, 3, 4, 3]),
            (Dominant, Eleventh) => Interval::from_semitones(&[4, 3, 3, 4, 4]),
            (Major, Eleventh) => Interval::from_semitones(&[4, 3, 4, 3, 3]),
            (Minor, Eleventh) => Interval::from_semitones(&[3, 4, 3, 4, 3]),
            (Dominant, Thirteenth) => Interval::from_semitones(&[4, 3, 3, 4, 3, 4]),
            (Major, Thirteenth) => Interval::from_semitones(&[4, 3, 4, 3, 3, 4]),
            (Minor, Thirteenth) => Interval::from_semitones(&[3, 4, 3, 4, 3, 4]),
            _ => Interval::from_semitones(&[4, 3]),
        }
        .unwrap()
    }

    /// Parse a chord using a regex.
    pub fn from_regex(string: &str) -> Result<Self, ChordError> {
        let (pitch_class, pitch_match) = PitchClass::from_regex(&string)?;

        let slash_option = string.find('/');
        let bass_note_result = if let Some(slash) = slash_option {
            PitchClass::from_regex(&string[slash + 1..].trim())
        } else {
            Err(NoteError::InvalidPitch)
        };

        let (quality, quality_match_option) = Quality::from_regex(
            &string[pitch_match.end()..slash_option.unwrap_or_else(|| string.len())].trim(),
        )?;

        let number = if let Some(quality_match) = quality_match_option {
            Number::from_regex(&string[quality_match.end()..])
                .unwrap_or((Triad, None))
                .0
        } else {
            Triad
        };
        let chord = Chord::new(pitch_class, quality, number);

        if let Ok((bass_note, _)) = bass_note_result {
            let inversion = chord
                .notes()
                .iter()
                .position(|note| note.pitch_class == bass_note)
                .unwrap_or(0);

            if inversion != 0 {
                return Ok(Chord::with_inversion(
                    pitch_class,
                    quality,
                    number,
                    inversion as u8,
                ));
            }
        }

        Ok(chord)
    }
}

impl Notes for Chord {
    fn notes(&self) -> Vec<Note> {
        let root_note = Note {
            octave: self.octave,
            pitch_class: self.root,
        };
        let mut notes = Interval::to_notes(root_note, self.intervals.clone());
        notes.rotate_left(self.inversion as usize);

        // Normalize to the correct octave
        if notes[0].octave > self.octave {
            let diff = notes[0].octave - self.octave;
            notes.iter_mut().for_each(|note| note.octave -= diff);
        }

        // Ensure that octave increments at the right notes
        for i in 1..notes.len() {
            if notes[i].pitch_class as u8 <= notes[i - 1].pitch_class as u8 {
                notes[i].octave = notes[i - 1].octave + 1;
            } else if notes[i].octave < notes[i - 1].octave {
                notes[i].octave = notes[i - 1].octave;
            }
        }
        notes
    }
}

impl Default for Chord {
    fn default() -> Self {
        Chord {
            root: PitchClass::C,
            octave: 4,
            intervals: vec![],
            quality: Quality::Major,
            number: Number::Triad,
            inversion: 0,
        }
    }
}
