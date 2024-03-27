# Procedural music generator

A simple program to generate procedural music.

The core of the program is based on [this tutorial](https://dev.to/deciduously/teaching-numbers-how-to-sing-3c8l) that I found when reading about music theory (you'll find my notes [here](MusicTheoryNotes.md), disclaimer though, it is not very well structured).

## Quick start

To use it, you'll need to install the Rust toolchains by following the instruction on the [official website](https://www.rust-lang.org/tools/install).

```bash
cargo run -- -h
```

## Future feature

Here are the future feature I want to (or try to) implement:
- [ ] Pattern generation (to make some kind of partition generation)
- [ ] Seed the RNG
- [ ] Sound envelopes ([short post about it](https://www.teachmeaudio.com/recording/sound-reproduction/sound-envelopes))
- [ ] Silence
- [ ] Syncopation
- [ ] Tempo modulation
- [ ] Create custom chord
- [ ] Check [this repository](https://github.com/andyherbert/ansiterm/tree/main/basic_waves/src) to implement different wave form.
