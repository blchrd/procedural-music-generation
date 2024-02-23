// Author: Ben Lovy <ben@deciduously.com>
// License: MIT
// Modified by Thomas Blanchard

use std::str::FromStr;

use crate::musictheory::{
    cent::Cent, char_strs, chord::{Chord, ChordInversion, ChordType}, chord_progression::ChordProgression, hertz::Hertz, interval::Interval, key::Key, mode::{Mode, PentatonicMode}, note::{self, Note, NoteLetter}, note_value::{NoteValue, NoteValueBase, NoteValueDotted}, piano_key::PianoKey, pitch::{Pitch, C_ZERO, MIDDLE_C}, scale::Scale, semitone::Semitone, tempo::Tempo
};

#[test]
fn test_substract_hertz() {
    assert_eq!(Hertz(440.0) - Hertz(1.0), Hertz(439.0))
}

#[test]
fn test_new_pitch() {
    assert_eq!(Pitch::default(), Pitch(Hertz(440.0)));
    assert_eq!(Pitch::new(MIDDLE_C), Pitch(Hertz(261.626)));
}

#[test]
fn test_new_piano_key() {
    use note::{NoteLetter, Accidental, Note};
    assert_eq!(
        PianoKey::default(),
        PianoKey {
            note: Note {
                letter: NoteLetter::C,
                accidental: None
            },
            octave: 0
        }
    );
    assert_eq!(
        PianoKey::new("A4").unwrap(),
        PianoKey {
            note: Note {
                letter: NoteLetter::A,
                accidental: None
            },
            octave: 4
        }
    );
    assert_eq!(
        PianoKey::new("Gb2").unwrap(),
        PianoKey {
            note: Note {
                letter: NoteLetter::G,
                accidental: Some(Accidental::Flat)
            },
            octave: 2
        }
    );
    assert_eq!(
        PianoKey::new("F#8").unwrap(),
        PianoKey {
            note: Note {
                letter: NoteLetter::F,
                accidental: Some(Accidental::Sharp)
            },
            octave: 8
        }
    );
}

#[test]
#[should_panic]
fn test_reject_piano_key_too_high() {
    assert_eq!(PianoKey::new("A9").unwrap(), PianoKey::default());
}

#[test]
#[should_panic]
fn test_reject_piano_key_invalid_letter() {
    assert_eq!(PianoKey::new("Q7").unwrap(), PianoKey::default());
}

#[test]
fn test_piano_key_to_str() {
    assert_eq!(PianoKey::default().to_string(), "C0".to_string());
    assert_eq!(PianoKey::new("A#4").unwrap().to_string(), "A#4".to_string());
    assert_eq!(PianoKey::new("Bb5").unwrap().to_string(), "Bb5".to_string());
}

#[test]
fn test_char_strs() {
        assert_eq!(char_strs("Hello"), ["H", "e", "l", "l", "o"])
}

#[test]
fn test_add_interval() {
    use Interval::*;
    assert_eq!(Unison + Unison, Unison);
    assert_eq!(Unison + Maj3, Maj3);
    assert_eq!(Maj2 + Min3, Perfect4);
    assert_eq!(Octave + Octave, Unison);
    assert_eq!(Tritone + Tritone, Unison);
    assert_eq!(Maj7 + Min3, Maj2);
}

#[test]
fn test_sub_interval() {
    use Interval::*;
    assert_eq!(Unison - Unison, Unison);
    assert_eq!(Unison - Maj3, Min6);
    assert_eq!(Maj2 - Min3, Maj7);
    assert_eq!(Octave - Octave, Unison);
    assert_eq!(Tritone - Tritone, Unison);
    assert_eq!(Maj7 - Min3, Min6);
}

#[test]
fn test_get_note_interval_from_c() {
    use Interval::*;
    use Note;
    assert_eq!(Note::from_str("A").unwrap().interval_from_c(), Maj6);
    assert_eq!(Note::from_str("A#").unwrap().interval_from_c(), Min7);
    assert_eq!(Note::from_str("Bb").unwrap().interval_from_c(), Min7);
    assert_eq!(Note::from_str("B").unwrap().interval_from_c(), Maj7);
    assert_eq!(Note::from_str("C").unwrap().interval_from_c(), Unison);
    assert_eq!(Note::from_str("C#").unwrap().interval_from_c(), Min2);
    assert_eq!(Note::from_str("D").unwrap().interval_from_c(), Maj2);
    assert_eq!(Note::from_str("D#").unwrap().interval_from_c(), Min3);
    assert_eq!(Note::from_str("E").unwrap().interval_from_c(), Maj3);
    assert_eq!(Note::from_str("F").unwrap().interval_from_c(), Perfect4);
    assert_eq!(Note::from_str("F#").unwrap().interval_from_c(), Tritone);
    assert_eq!(Note::from_str("G").unwrap().interval_from_c(), Perfect5);
    assert_eq!(Note::from_str("G#").unwrap().interval_from_c(), Min6);
}

#[test]
fn test_get_note_offset() {
    use Interval::*;
    use Note;
    let a = Note::from_str("A").unwrap();
    assert_eq!(Note::from_str("A").unwrap().get_offset(a), Unison);
    assert_eq!(Note::from_str("A#").unwrap().get_offset(a), Min2);
    assert_eq!(Note::from_str("B").unwrap().get_offset(a), Maj2);
    assert_eq!(Note::from_str("C").unwrap().get_offset(a), Min3);
    assert_eq!(Note::from_str("C#").unwrap().get_offset(a), Maj3);
    assert_eq!(Note::from_str("D").unwrap().get_offset(a), Perfect4);
    assert_eq!(Note::from_str("D#").unwrap().get_offset(a), Tritone);
    assert_eq!(Note::from_str("E").unwrap().get_offset(a), Perfect5);
    assert_eq!(Note::from_str("F").unwrap().get_offset(a), Min6);
    assert_eq!(Note::from_str("F#").unwrap().get_offset(a), Maj6);
    assert_eq!(Note::from_str("G").unwrap().get_offset(a), Min7);
    assert_eq!(Note::from_str("G#").unwrap().get_offset(a), Maj7);
}

#[test]
fn test_add_interval_to_note() {
    use Interval::*;
    use Note;
    let a = Note::from_str("A").unwrap();
    assert_eq!(a + Unison, a);
    assert_eq!(a + Min2, Note::from_str("A#").unwrap());
    assert_eq!(a + Maj2, Note::from_str("B").unwrap());
    assert_eq!(a + Min3, Note::from_str("C").unwrap());
    assert_eq!(a + Maj3, Note::from_str("C#").unwrap());
    assert_eq!(a + Perfect4, Note::from_str("D").unwrap());
    assert_eq!(a + Tritone, Note::from_str("D#").unwrap());
    assert_eq!(a + Perfect5, Note::from_str("E").unwrap());
    assert_eq!(a + Min6, Note::from_str("F").unwrap());
    assert_eq!(a + Maj6, Note::from_str("F#").unwrap());
    assert_eq!(a + Min7, Note::from_str("G").unwrap());
    assert_eq!(a + Maj7, Note::from_str("G#").unwrap());
}

#[test]
fn test_note_letter_to_interval() {
    use Interval::*;
    use NoteLetter::*;
    assert_eq!(C.interval_from_c(), Unison);
    assert_eq!(D.interval_from_c(), Maj2);
    assert_eq!(E.interval_from_c(), Maj3);
    assert_eq!(F.interval_from_c(), Perfect4);
    assert_eq!(G.interval_from_c(), Perfect5);
    assert_eq!(A.interval_from_c(), Maj6);
    assert_eq!(B.interval_from_c(), Maj7);
}

#[test]
fn test_c_major() {
    assert_eq!(
        &Key::new(Scale::default(), PianoKey::default(), 1).to_string(),
        "[ C D E F G A B C ]"
    )
}

#[test]
fn test_a_major() {
    assert_eq!(
        &Key::new(Scale::default(), PianoKey::from_str("A4").unwrap(), 1).to_string(),
        "[ A B C# D E F# G# A ]"
    )
}

#[test]
fn test_g_major() {
    assert_eq!(
        &Key::new(Scale::default(), PianoKey::from_str("G4").unwrap(), 1).to_string(),
        "[ G A B C D E F# G ]"
    )
}

#[test]
fn test_a_minor() {
    use Mode::*;
    use Scale::*;

    assert_eq!(
        &Key::new(Diatonic(Aeolian), PianoKey::from_str("A4").unwrap(), 1).to_string(),
        "[ A B C D E F G A ]"
    )
}

#[test]
fn test_pentatonic_c_major() {
    use PentatonicMode::*;
    use Scale::*;

    assert_eq!(
        &Key::new(Pentatonic(Major), PianoKey::from_str("C4").unwrap(), 1).to_string(),
        "[ C D E G A C ]"
    )
}

#[test]
fn test_pentatonic_a_minor() {
    use PentatonicMode::*;
    use Scale::*;

    assert_eq!(
        &Key::new(Pentatonic(Minor), PianoKey::from_str("A4").unwrap(), 1).to_string(),
        "[ A C D E G A ]"
    )
}

#[test]
fn test_pentatonic_c_suspended() {
    use PentatonicMode::*;
    use Scale::*;

    assert_eq!(
        &Key::new(Pentatonic(Suspended), PianoKey::from_str("C4").unwrap(), 1).to_string(),
        "[ C D F G A# C ]"
    )
}

#[test]
fn test_pentatonic_e_blues_minor() {
    use PentatonicMode::*;
    use Scale::*;
    assert_eq!(
        &Key::new(Pentatonic(BluesMinor), PianoKey::from_str("E4").unwrap(), 1).to_string(),
        "[ E G A C D E ]"
    )
}

#[test]
fn test_semitones_to_cents() {
    assert_eq!(Cent::from(Semitone(1)), Cent(100.0));
    assert_eq!(Cent::from(Semitone(12)), Cent(1200.0));
}

#[test]
fn test_interval_to_cents() {
    use Interval::*;
    assert_eq!(Cent::from(Unison), Cent(0.0));
    assert_eq!(Cent::from(Min2), Cent(100.0));
    assert_eq!(Cent::from(Octave), Cent(1200.0));
}

#[test]
fn test_add_cents_to_pitch() {
    let mut pitch = Pitch::default();
    pitch += Cent(3.9302);
    assert_eq!(pitch, Pitch::new(Hertz(441.0)));
}

#[test]
fn test_add_semitones_to_pitch() {
    use Interval::Octave;
    let mut pitch = Pitch::default();
    pitch += Semitone::from(Octave);
    assert_eq!(pitch, Pitch::new(Hertz(880.0)))
}

#[test]
fn test_add_interval_to_pitch() {
    use Interval::Min2;
    let mut pitch = Pitch::default();
    pitch += Min2;
    assert_eq!(pitch, Pitch::new(Hertz(466.1)))
}

#[test]
fn test_piano_key_to_pitch() {
    assert_eq!(Pitch::from(PianoKey::new("A4").unwrap()), Pitch::default());
    assert_eq!(Pitch::from(PianoKey::default()), Pitch::new(C_ZERO));
}

#[test]
fn test_tempo_bps() {
    assert_eq!(Tempo::from(60).get_bps(), 1.0);
    assert_eq!(Tempo::from(30).get_bps(), 0.5);
    assert_eq!(Tempo::from(120).get_bps(), 2.0);
    assert_eq!(Tempo::from(90).get_bps(), 1.5);
}

#[test]
fn test_chord_interval() {
    use Interval::*;
    use ChordType::*;
    assert_eq!(MajorSixth.get_intervals(), vec![Maj3, Perfect5, Maj6]);
    assert_eq!(MinorSeventh.get_intervals(), vec![Min3, Perfect5, Min7]);
    assert_eq!(DiminishedTriad.get_intervals(), vec![Min3, Tritone]);
}

#[test]
fn test_c4_major_triad() {
    use ChordType::*;
    use ChordInversion::Root;
    assert_eq!(
        &Chord::new(MajorTriad, PianoKey::from_str("C4").unwrap(), Root).get_keys_string(),
        "| C4 E4 G4 |"
    );
}

#[test]
fn test_c4_minor_triad() {
    use ChordType::*;
    use ChordInversion::Root;
    assert_eq!(
        &Chord::new(MinorTriad, PianoKey::from_str("C4").unwrap(), Root).get_keys_string(),
        "| C4 D#4 G4 |"
    );
}

#[test]
fn test_c4_augmented_triad() {
    use ChordType::*;
    use ChordInversion::Root;
    assert_eq!(
        &Chord::new(AugmentedTriad, PianoKey::from_str("C4").unwrap(), Root).get_keys_string(),
        "| C4 E4 G#4 |"
    );
}

#[test]
fn test_g_sharp3_diminished_seventh() {
    use ChordType::*;
    use ChordInversion::Root;
    assert_eq!(
        &Chord::new(DiminishedSeventh, PianoKey::from_str("G#4").unwrap(), Root).get_keys_string(),
        "| G#4 B4 D5 F5 |"
    );
}

#[test]
fn test_e3_power_triad() {
    use ChordType::PowerTriad;
    use ChordInversion::Root;
    assert_eq!(
        &Chord::new(PowerTriad, PianoKey::from_str("E3").unwrap(), Root).get_keys_string(),
        "| E3 B3 E4 |"
    )
}

#[test]
fn test_add_interval_to_piano_key() {
    use Interval::{Min2, Min3, Maj3, Octave};
    let sp = PianoKey::from_str("B3").unwrap();
    assert_eq!((sp + Min2).to_string(), "C4");
    assert_eq!((sp + Min3).to_string(), "D4");
    assert_eq!((sp + Maj3).to_string(), "D#4");
    assert_eq!((sp + Octave).to_string(), "B4");
}

#[test]
fn test_c4_major_triad_first_inversion() {
    use ChordType::MajorTriad;
    use ChordInversion::First;

    assert_eq!(
        &Chord::new(MajorTriad, PianoKey::from_str("C4").unwrap(), First).get_keys_string(),
        "| E4 G4 C5 |"
    );
}

#[test]
fn test_c4_augmented_second_inversion() {
    use ChordType::AugmentedTriad;
    use ChordInversion::Second;
    assert_eq!(
        &Chord::new(AugmentedTriad, PianoKey::from_str("C4").unwrap(), Second).get_keys_string(),
        "| G#4 C5 E5 |"
    );
}

#[test]
fn test_chord_i_v_vi_iv_c_major_scale() {
    assert_eq!(
        &ChordProgression::default().to_string(),
        "[ C4maj G4maj A4min F4maj ]"
    );
}

// Test for note value
#[test]
fn test_note_value_to_str() {
    use NoteValueBase::*;
    use NoteValueDotted::*;
    assert_eq!(NoteValue::default().to_string(), "Quarter note");
    assert_eq!(NoteValue{base: Whole, dotted: None}.to_string(), "Whole note");
    assert_eq!(NoteValue{base: Half, dotted: Some(Dotted)}.to_string(), "Half dotted note");
    assert_eq!(NoteValue{base: Eighth, dotted: Some(DoubleDotted)}.to_string(), "Eighth double dotted note");
}

#[test]
fn test_note_value_relative_duration() {
    use NoteValueBase::*;
    use NoteValueDotted::*;
    assert_eq!(NoteValue::default().get_relative_duration(), 0.25);
    assert_eq!(NoteValue{base: Whole, dotted: None}.get_relative_duration(), 1.0);
    assert_eq!(NoteValue{base: Half, dotted: Some(Dotted)}.get_relative_duration(), 0.75);
    assert_eq!(NoteValue{base: Whole, dotted: Some(DoubleDotted)}.get_relative_duration(), 1.75);
}

#[test]
fn test_note_value_duration_in_second() {
    use NoteValueBase::*;
    use NoteValueDotted::*;
    assert_eq!(NoteValue::default().get_duration_for_tempo(Tempo::from(60)), 1.0);
    assert_eq!(NoteValue::default().get_duration_for_tempo(Tempo::from(120)), 0.5);
    assert_eq!(NoteValue{base: Half, dotted: None}.get_duration_for_tempo(Tempo::from(120)), 1.0);
    assert_eq!(NoteValue{base: Whole, dotted: Some(Dotted)}.get_duration_for_tempo(Tempo::from(60)), 6.0);
}