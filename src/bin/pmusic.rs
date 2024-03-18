use std::time::{Duration, Instant};

use pmusic::{
    musicgeneration::{chord_progression_generator::chord_progression_generation, rhythm_pattern_generator::rhythm_pattern_generation_for_chord, sheet_generator::sheet_generation}, 
    musicsource::{chord_music_maker::ChordMusicMaker, sheet_music_maker::SheetMusicMaker}, 
    musictheory::{chord_progression::ChordProgression, piano_key::PianoKey, scale::Scale, time_signature::TimeSignature}
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
    let mut nb_measures = 4;

    let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44_100);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    if opt.chord_mode {
        let time_signature = TimeSignature::default();
        let mut chord_base_note = opt.base_note;
        chord_base_note.octave -= 2;
        let chord_progression = ChordProgression::from_scale_and_str(
            opt.scale,
            chord_base_note,
            &chord_progression_generation(opt.scale, time_signature.clone())
        );
        let rhythm_pattern = rhythm_pattern_generation_for_chord(time_signature.clone());
        let chords = ChordMusicMaker::new(
            chord_progression.clone(),
            rhythm_pattern.clone(),
            opt.tempo,
        );

        nb_measures = chord_progression.clone().chords.len();

        // By removing the .amplify at the end, we can make the sound saturate
        controller.add(chords.take_duration(Duration::from_secs(opt.duration)).amplify(amplify_value));
    }

    let music = SheetMusicMaker::new(sheet_generation(opt.base_note, opt.scale, opt.octaves, nb_measures as i32), opt.tempo);
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
