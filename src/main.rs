use clap::{load_yaml, App};
use median::heap::Filter as MedianFilter;
use melody_extraction as me;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(i) = matches.value_of("INPUT") {
        if let Some(ref matches) = matches.subcommand_matches("salamon") {
            let audio_path = String::from(i);
            let audio_signal = me::get_audio_signal(&audio_path);
            let melody = me::salamon(&audio_signal);
            let mut median_filter = MedianFilter::<f32>::new(86);
            let melody: Vec<f32> = melody.iter().map(|&x| median_filter.consume(x)).collect();

            if let Some(p) = matches.value_of("PLOTFILE") {
                let plot_path = String::from(p);
                let _ = me::plot_results(&melody, &audio_path, &plot_path);
            }
            if let Some(m) = matches.value_of("MIDIFILE") {
                let tempo = me::get_tempo(&audio_signal);
                let midi_path = String::from(m);
                me::melody_to_midi(&melody, tempo, &midi_path);
            }
        }
    } else {
        panic!("No input audio path detected");
    }
}
