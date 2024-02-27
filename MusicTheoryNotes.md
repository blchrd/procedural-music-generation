#dev #music #rust

# Procedural music in rust

## Musical theory

### Notes

#### Frequency

A4 = 440hz

The following formula gets the frequency of a note compare to the 440hz A4, h is the half-steps above or below that note.
f = 2^(h/12) * 440

So if A4 = 440, to get B4, we need to add 2 half-steps (or semi-tones), it's almost equal to 494hz (493.88).

If we need to go from B4 to another note, we use the same formula, and change the frequency:

f = 2^(h/12) * 493.88

#### MIDI

A MIDI note is a integer between 0 (C-1) and 127 (G9). One unit correspond to a semi-tone.

A4 = 69 (nice), therefore B4 = 71, C5 = 73 and so on.

70 is a A4# (or B4b).

#### Note reference

* A = la 
* B = si 
* C = do (octave change at C, we go from B4 to C5, or from C4 to B3)
* D = ré 
* E = mi
* F = fa 
* G = sol

A sharp raises the pitch of a note by one semitone, it is noted A#.

A flat lowers the pitch of a note by one semitone, is is noted Ab.

A F# is almost equivalent to a Gb.

There are others notation, based on the key signature, but we don't get into details here, the key signature will not change for this project.

E# and B# doesn't not exist (as well as Fb and Cb), so if we go from E to F, we only add a semi-tone, not a full tone.

### Chords

#### Intervals

|Interval|Notation|Semi-tones|Example|Harmony|
|:--:|:--:|:--:|:--:|:--:|
|Minor 2nd|m2|1|C-C#|dis|
|Major 2nd|M2|2|C-D|dis|
|Minor 3rd|m3|3|C-D#|con|
|Major 3rd|M3|4|C-E|con|
|Perfect 4th|P4|5|C-F|con|
|Augmented 4th (Tritone)|d5|6|C-F#|dis|
|Diminished 5th (Tritone)|d5|6|C-Gb|dis|
|Perfect 5th|P5|7|C-G|con|
|Minor 6th|m6|8|C-Ab|con|
|Major 6th|M6|9|C-A|con|
|Minor 7th|m7|10|C-Bb|dis|
|Major 7th|M7|11|C-B|dis|
|Octave|12|O|C-C'|con|

#### Chord types

##### Common

|Name|Symbol (on C)|Components (intervals) (exept fundamental)|
|---|:--:|:--:|
|Major triad|C|M3-P5|
|Major sixth chord|Cmaj6|M3-P5-M6|
|Dominant seventh chord|Cdom7|M3-P5-m7|
|Augmented triad|Caug|M3-A5|
|Augmented seventh chord|Caug7|M3-A5-m7|
|Minor triad|Cmin|m3-P5|
|Minor sixth chord|Cmin6|m3-P5-M6|
|Minor seventh chord|Cmin7|m3-P5-m7|
|Minor-major seventh chord|Cmin/maj7|m3-P5-M7|
|Diminished triad|Cdim|m3-d5|
|Diminished seventh chord|Cdim7|m3-d5-d7|
|Half-diminished seventh chord|Cø7|m3-d5-m7|

#### Chord progression

A common chord progression is: `I-V-vi-IV`, it is roman number corresponding to a given scale (I is the first note of the scale, it's called a degree).

If the roman number is upper case, we take a major chord, if it is lower case, we take minor chord.

If the degree is followed by `°`, it is a diminished chord, by `ø`, it's half-diminished.

Then we can add `6` or `7` at the end for sixth or seventh chord.

Power chord didn't really have notation here, I have to think about that during implementation.

[https://en.wikipedia.org/wiki/Chord_progression](https://en.wikipedia.org/wiki/Chord_progression)
[https://en.wikipedia.org/wiki/Function_(music)](https://en.wikipedia.org/wiki/Function_(music))
[https://en.wikipedia.org/wiki/List_of_chord_progressions](https://en.wikipedia.org/wiki/List_of_chord_progressions)

Classic chord progression use diatonic scale (7 notes). For the major scales, it is straightforward, but a little tricky for minor scale (or at least, I didn't understand how it works).

For a major scale, the chord degrees are:
- I (Tonic)
- ii (Supertonic)
- iii (Mediant)
- IV (Subdominant)
- V (Dominant)
- vi (Submediant)
- vii° (Leading tone): Diminished chord

For a minor scale, chord degrees are:
- i
- ii° ou ii (diminished or minor chord, depend on scale?)
- III or ii (major or minor chord, depend on scale?)
- iv
- V or v (major or minor chord, depend on scale?)
- VI or vi (major or minor chord, depend on scale?)
- vii° or vii (diminished or minor, depend on scale?)

### Scales

#### Type of scale

* Chromatic: 12 notes per octave (example: C-C#-D-D#-E-F-F#-G-G#-A-A#-B)
* Nonatonic: 9 notes per octave (in blues)
* Octatonic: 8 notes per octave (jazz and modern classical)
* Heptatonic: 7 notes per octave (most common modern western scale)
* Diatonic: heptatonic scale with only tone and semi-tone pitch (can't have a tone and a half jump between two notes in the scale)
* Hexatonic: 6 notes per octave (western folk music)
* Pentatonic: 5 notes per octave (asian music)
* Tetratonic / tritonic et ditonic: "primitive" music

#### Construction

The following list contains the tone interval for different type of scale. To use it, we need to choose a key (the starting note of the scale), and going through the list of intervals, the first interval is the key note of the scale, the last one allows us to check if we go back to the original key note.

By example, for the A3 major pentatonic scale, we'll have A3-B3-C4-E4-F4, and we repeat if we want higher pitched note.

### Tempo

#### BPM

BPM is pretty easy to understand, 60 BPM is one beat per second, so a tempo of 120 BPM means 2 beat per second, 1 beat every 50ms.

#### Time signature

The time signature indicate the the number of note into a musical measure. For a 4/4 signature, the measure contains 4 beat, for 3/4, it contains 3 beats.

y/x, x indicates the note value that the signature is counting, it is always a power of 2. If it's 4, we counting quarter note, if it's eight, we couting eighth-note. While y indicate the number of note value the measure contains, by example, a 4/8 signature means we have 4 eighth-notes by measure.

We can have x being something else than a power of two, but in our case, we'll assume these time signatures are not valid.

In the beginning, we'll use 4/4 signature.

#### Common rhythm pattern

For measure generation, here is the most common rhythm patterns, after implementing them, we can improve the generation.

For 4/4 time signature:
- 𝅘𝅥 𝅘𝅥 𝅘𝅥 𝅘𝅥𝅮 𝅘𝅥𝅮 (rock ballad type)
- 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥 𝅘𝅥𝅮 𝅘𝅥
- 𝅘𝅥𝅮 𝅘𝅥 𝅘𝅥 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥𝅮
- 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥𝅮
- 𝅘𝅥 𝅘𝅥𝅮 𝅘𝅥 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥𝅮
- 𝅘𝅥. 𝅘𝅥 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥𝅮

For 3/4 time signature:
- 𝅘𝅥 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥𝅮
- 𝅘𝅥𝅮 𝅘𝅥𝅮 𝅘𝅥 𝅘𝅥𝅮 𝅘𝅥𝅮

#### Note values

* Whole note: ronde, 𝅝
* Half note: blanche, 𝅗𝅥
* Quarter note: noire, 𝅘𝅥
* Eighth note: croche, 𝅘𝅥𝅮
* Sixteenth note: double-croche, 𝅘𝅥𝅯
* Thirty-second note: triple-croche, 𝅘𝅥𝅰
* Sixty-fourth note: quadruple-croche, 𝅘𝅥𝅱
* etc.

##### Dotted note

A dotted note add 1/2 of the duration to the note. We can double or triple the dot.

- Dotted note: +1/2 of the note duration
- Double dotted note: +1/2 +1/4 of the note duration
- Triple dotted note: +1/2 +1/4 +1/8 of the note duration

I'm not sure we'll use something other than simple dotted note.

##### Tuplets

A tuplet is a rhythm that involve dividing the beat into a different number of equal subdivision.

Meaning, by example: 1 quarter note can be divided into 2 eighth notes, or 4 sixteenth notes. But, if for any reason we want to divide this quarter note in 3, then we use a tuplet, here a triplet, and we have 3 triplets eighth notes, equivalent to 1 quarter note.

The most tuplets I came across while composing music is triplet and sextuplet, but all the other division exist as well.

I'm not sure we'll use this division of note, but it'll be implemented (at least, we'll have it if we need it).