use std::{io::Error, time::{Duration, Instant}};

use pmusic::{
    musicgeneration::{
        chord_progression_generator::chord_progression_generation, random_scale::{get_random_base_note, get_random_scale}, rhythm_pattern_generator::rhythm_pattern_generation_for_chord, sheet_from_binary::sheet_from_binary_file, sheet_generator::sheet_generation
    }, 
    musicsource::{chord_music_maker::ChordMusicMaker, sheet_music_maker::SheetMusicMaker}, 
    musictheory::{chord_progression::ChordProgression, key::Key, piano_key::PianoKey, scale::Scale, time_signature::TimeSignature}, signal::adsr_envelop::AdsrEnvelop
};
use rodio::{dynamic_mixer, OutputStream, Sink, Source};
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pmusic")]
struct Opt {
    /// Add chord progression in addition of the melody
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
    /// Instead of playing the track, output the result in ./output/output.wav
    #[structopt(short, long)]
    file_out: bool,
    /// WARNING: this flag can produce some very high pitch sound
    #[structopt(short, long)]
    instrument_debug: bool,
    /// Will pick a rhythm in a short list of common rhythm pattern
    #[structopt(short, long)]
    use_common_pattern: bool,
    /// Completely randomize the chord progression
    #[structopt(short="w", long)]
    random_chord_progression: bool,
    /// Randomize scale and base note (on the fourth octave)
    #[structopt(short="r", long)]
    full_random: bool,
    /// Seed for random generation
    #[structopt(long, default_value = "0")]
    seed: u64,
    /// Source binary file to generate the sheet (will ignore all other generation arguments)
    #[structopt(short="i",long, default_value = "")]
    file_in: String,
    /// Parse the file half a byte at a time to generate the sheet (only with source binary file)
    #[structopt(short="h", long)]
    half_byte_parsing: bool,
}

fn main() -> Result<(), Error> {
    let now = Instant::now();
    let opt = Opt::from_args();
    
    if opt.file_in != "" {
        let sheet = sheet_from_binary_file::<Error>(
            opt.base_note, 
            &opt.file_in,
            opt.half_byte_parsing,
        )?;
        let music = SheetMusicMaker::new(
        sheet, 
        opt.tempo, 
        opt.instrument_debug)
                // .set_adsr_envelop(AdsrEnvelop::new(0.1, 0.2, 1.5, 0.4));
                .set_adsr_envelop(AdsrEnvelop::default());
        let amplify_value = 0.2;
        let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44_100);
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
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
            controller.add(music.take_duration(Duration::from_secs(opt.duration)).amplify(amplify_value));
            sink.append(mixer);
            sink.sleep_until_end();
        }
        return Ok(())
    }
    
    // If using a source binary file, following code is never executed
    let seed: u64;
    if opt.seed == 0 {
        let mut seed_gen = SmallRng::from_entropy();
        seed = seed_gen.next_u64();
    } else {
        seed = opt.seed;
    }
    
    let mut rng_seed = SmallRng::seed_from_u64(seed);
    let amplify_value;
    if opt.instrument_debug {
        amplify_value = 0.1;
    } else {
        amplify_value = 0.2;
    }
    let mut nb_measures = 4;

    let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44_100);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let scale: Scale;
    let base_note: PianoKey;
    if opt.full_random {
        scale = get_random_scale(&mut rng_seed);
        base_note = get_random_base_note(&mut rng_seed)
    } else {
        scale = opt.scale;
        base_note = opt.base_note;
    }

    println!("Seed: {}", seed);
    println!("Scale: {} {} {}", base_note, scale, Key::new(opt.scale, opt.base_note, opt.octaves));
    if opt.chord_mode {
        let time_signature = TimeSignature::default();
        let mut chord_base_note = opt.base_note;
        chord_base_note.octave = 2;
        let chord_progression = ChordProgression::from_scale_and_str(
            opt.scale,
            chord_base_note,
            &chord_progression_generation(opt.scale, time_signature.clone(), opt.random_chord_progression, &mut rng_seed)
        );
        let rhythm_pattern = rhythm_pattern_generation_for_chord(time_signature.clone(), &mut rng_seed);
        let chords = ChordMusicMaker::new(
            chord_progression.clone(),
            rhythm_pattern.clone(),
            opt.tempo,
            opt.instrument_debug,
        )
        //.set_adsr_envelop(AdsrEnvelop::new(0.1, 0.2, 4.0, 1.0));
        .set_adsr_envelop(AdsrEnvelop::default());

        nb_measures = chord_progression.clone().chords.len();
        println!("Chord progression: {}", chord_progression);

        // By removing the .amplify at the end, we can make the sound saturate
        controller.add(chords.take_duration(Duration::from_secs(opt.duration)).amplify(amplify_value - 0.05));
    }

    let music = SheetMusicMaker::new(
        sheet_generation(
            opt.base_note, 
            opt.scale, 
            opt.octaves, 
            nb_measures as i32,
            opt.use_common_pattern,
            &mut rng_seed
        ), 
        opt.tempo, 
        opt.instrument_debug)
                // .set_adsr_envelop(AdsrEnvelop::new(0.1, 0.2, 1.5, 0.4));
                .set_adsr_envelop(AdsrEnvelop::default());
    println!("{}", music);
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
        controller.add(music.take_duration(Duration::from_secs(opt.duration)).amplify(amplify_value));
        sink.append(mixer);
        sink.sleep_until_end();
    }

    Ok(())
}