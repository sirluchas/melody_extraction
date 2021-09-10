use minimp3::{Decoder, Error, Frame};
use std::fs::File;

use aubio_rs::*;

use ghakuf::messages::*;
use ghakuf::writer::*;
use std::path;

pub fn get_audio_signal(file_path: &str) -> Vec<f32> {
    let mut decoder = Decoder::new(File::open(file_path).unwrap());
    let mut audio_signal: Vec<f32> = Vec::new();

    loop {
        match decoder.next_frame() {
            Ok(Frame { data, .. }) => {
                audio_signal.extend(data.into_iter().map(|d| d as f32 * (1. / 32768.) as f32));
            }
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    if cfg!(debug_assertions) {
        audio_signal = audio_signal[0..{
            if audio_signal.len() < 44100 {
                audio_signal.len()
            } else {
                44100
            }
        }]
            .to_vec();
    }

    audio_signal
}

pub fn peak_indices<T: PartialOrd>(arr: &[T]) -> Vec<usize> {
    let mut out = Vec::new();
    let alen = arr.len();

    if alen > 2 {
        for i in 1..alen - 1 {
            if arr[i - 1] < arr[i] && arr[i + 1] < arr[i] {
                out.push(i);
            }
        }
    }

    out
}

pub fn get_tempo(audio_signal: &[f32]) -> f32 {
    let mut tempo_obj = Tempo::new(OnsetMode::Phase, 8192, 128, 44100).unwrap();
    let samples: Vec<Smpl> = audio_signal.iter().map(|&x| x as Smpl).collect();
    let mut ans = 0.;

    for block in samples.windows(8192).step_by(128) {
        if block.len() == 8192 {
            let _ = tempo_obj.do_result(block);
            let tempo = tempo_obj.get_bpm();
            if tempo != 0. {
                ans = tempo;
            }
        }
    }

    ans
}

pub fn melody_to_midi(melody: &[f32], bpm: f32, file_path: &str) {
    let bpm = bpm.round();
    let midi_tempo: u32 = 60 * 1000000 / bpm as u32;
    let delta_conv = midi_tempo as f32 * 1e-6 / 480.;
    let threshold_t = (0.1 / delta_conv) as u32;

    let mut write_messages: Vec<Message> = vec![Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::SetTempo,
        data: [
            (midi_tempo >> 16) as u8,
            (midi_tempo >> 8) as u8,
            midi_tempo as u8,
        ]
        .to_vec(),
    }];
    let mut prev_midi = 0;
    let mut prev_time = 0;

    for (t, freq) in melody.iter().enumerate() {
        let midi_note = freq_to_midi(*freq as Smpl) as u32;
        if prev_midi != midi_note {
            let curr_d = (t - prev_time) as f32 * 128. / 44100.;
            let curr_d = (curr_d / delta_conv) as u32;
            if curr_d < threshold_t {
                write_messages.pop();
            } else {
                write_messages.push(Message::MidiEvent {
                    delta_time: curr_d,
                    event: MidiEvent::NoteOff {
                        ch: 0,
                        note: prev_midi as u8,
                        velocity: 0x7f,
                    },
                });
            }
            write_messages.push(Message::MidiEvent {
                delta_time: 0,
                event: MidiEvent::NoteOn {
                    ch: 0,
                    note: midi_note as u8,
                    velocity: 0x7f,
                },
            });
            prev_midi = midi_note;
            prev_time = t;
        }
    }
    
    let last_t = melody.len() - 1;
    let last_d = (last_t - prev_time) as f32 * 128. / 44100.;
    let last_d = (last_d / delta_conv) as u32;
    if prev_time != last_t {
        write_messages.push(Message::MidiEvent {
            delta_time: last_d,
            event: MidiEvent::NoteOff {
                ch: 0,
                note: prev_midi as u8,
                velocity: 0x7f,
            },
        });
    }
    write_messages.push(Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::EndOfTrack,
        data: Vec::new(),
    });

    let path = path::Path::new(file_path);
    let mut writer = Writer::new();
    writer.running_status(true);
    for message in &write_messages {
        writer.push(&message);
    }
    if let Err(e) = writer.write(&path) {
        println!("{:?}", e);
    }
}
