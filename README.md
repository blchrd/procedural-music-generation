# Procedural music generator

A simple program to generate procedural music.

The core of the program is based on [this tutorial](https://dev.to/deciduously/teaching-numbers-how-to-sing-3c8l) that I found when reading about music theory (you'll find my notes [here](MusicTheoryNotes.md), disclaimer though, it is not very well structured).

## Quick start

To do that, you'll need to install the Rust toolchains by following the instruction on the [official website](https://www.rust-lang.org/tools/install).

```bash
cargo run -- -h
```

## Future feature

Here are the future feature I want to implement:
- [ ] Barre chord?
- [ ] Tempo modulation
- [x] Export to files
- [x] Note duration
- [ ] Implementation of silence
- [ ] Syncopation
- [ ] Create custom chord
- [ ] Pattern generation (to make some kind of partition generation)
- [ ] Check [this repository](https://github.com/andyherbert/ansiterm/tree/main/basic_waves/src) to implement different wave form.
