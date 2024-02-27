use std::time::{Duration, Instant};

use pmusic::{
    musicgeneration::sheet_generator::sheet_generation, 
    musicsource::{chord_music_maker::ChordMusicMaker, melody_music_maker::MelodyMusicMaker, sheet_music_maker::SheetMusicMaker}, 
    musictheory::{chord_progression::ChordProgression, piano_key::PianoKey, scale::Scale}
};
use rodio::{dynamic_mixer, OutputStream, Sink, Source};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pmusic")]
struct Opt {
    #[structopt(short, long)]
    chord_mode: bool,
    #[structopt(short, long, default_value="C4")]
    base_note: PianoKey,
    #[structopt(short, long, default_value = "Ionian")]
    scale: Scale,
    #[structopt(short, long, default_value = "1")]
    octaves: u8,
    #[structopt(short, long, default_value = "60")]
    tempo: u16,
    #[structopt(short, long, default_value = "10")]
    duration: u64,
    #[structopt(short, long)]
    file_out: bool
}

fn main() {
    let now = Instant::now();
    let opt = Opt::from_args();
    let amplify_value = 0.2;

    let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 48_000);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    if opt.chord_mode {
        let mut chord_base_note = opt.base_note;
        chord_base_note.octave -= 2;
        let chords = ChordMusicMaker::new(
            ChordProgression::from_scale_and_str(opt.scale, chord_base_note, "I-V-vi-IV"), 
            opt.tempo / 2
        );
        // By removing the .amplify at the end, we can make the sound saturate
        controller.add(chords.take_duration(Duration::from_secs(opt.duration)).amplify(amplify_value));
    }

    // let music = MelodyMusicMaker::new(opt.base_note, opt.scale, opt.octaves, opt.tempo);
    let music = SheetMusicMaker::new(sheet_generation(opt.base_note, opt.scale, opt.octaves), opt.tempo);

    if opt.file_out {
        let filepath = "./output/output.wav";
        println!("Export to {}", filepath);
        controller.add(music.take_duration(Duration::from_secs(opt.duration)).amplify(amplify_value));
        let head = wav_io::new_stereo_header();
        let mut file_out = std::fs::File::create(filepath).unwrap();
        wav_io::write_to_file(&mut file_out, &head, &mixer.convert_samples().into_iter().collect::<Vec<f32>>()).unwrap();

        // "benchmark"
        let elapsed_time = now.elapsed();
        println!("Execution took {} seconds.", elapsed_time.as_secs());
    } else {
        println!("{}", music);
        controller.add(music.take_duration(Duration::from_secs(opt.duration)).amplify(amplify_value));
        sink.append(mixer);
        sink.sleep_until_end();
    }
}
