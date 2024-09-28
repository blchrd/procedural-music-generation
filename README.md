# Procedural music generator

A simple program to generate procedural music.

The core of the program is based on [this tutorial](https://dev.to/deciduously/teaching-numbers-how-to-sing-3c8l) that I found when reading about music theory (you'll find my notes [here](MusicTheoryNotes.md), disclaimer though, it is not very well structured).

## Quick start

To use it, you'll need to install the Rust toolchains by following the instruction on the [official website](https://www.rust-lang.org/tools/install).

```bash
cargo run -- -h
```

If you just want to give it a try, you can use the following command
```bash
cargo run --release -- -c -r -d 120 -o 2
```

## Future feature

No GUI is planned right now, but if you want to build one because CLI isn't your thing, please do!

You can find the TODO file [here](TODO.md)
