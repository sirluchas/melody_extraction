# CREATED: 11/9/15 3:57 PM by Justin Salamon <justin.salamon@nyu.edu>

import vamp
import argparse
import numpy as np
from midiutil.MidiFile import MIDIFile
from scipy.signal import medfilt
import librosa

"""
Extract the melody from an audio file and convert it to MIDI.

The script extracts the melody from an audio file using the Melodia algorithm,
and then segments the continuous pitch sequence into a series of quantized
notes, and exports to MIDI using the provided BPM.

Examples:
python audio_to_midi_melodia.py ~/song.wav ~/song.mid
"""


def save_midi(outfile, notes, tempo):
    track = 0
    time = 0
    midifile = MIDIFile(1)

    # Add track name and tempo.
    midifile.addTrackName(track, time, "MIDI TRACK")
    midifile.addTempo(track, time, tempo)

    channel = 0
    volume = 100

    for note in notes:
        onset = note[0] * (tempo / 60.0)
        duration = note[1] * (tempo / 60.0)
        pitch = note[2]
        pitch = pitch.__int__()
        midifile.addNote(track, channel, pitch, onset, duration, volume)

    # And write it to disk.
    binfile = open(outfile, "wb")
    midifile.writeFile(binfile)
    binfile.close()


def midi_to_notes(midi, fs, hop, smooth, minduration):
    # smooth midi pitch sequence first
    if smooth > 0:
        filter_duration = smooth  # in seconds
        filter_size = int(filter_duration * fs / float(hop))
        if filter_size % 2 == 0:
            filter_size += 1
        midi_filt = medfilt(midi, filter_size)
    else:
        midi_filt = midi

    notes = []
    p_prev = 0
    duration = 0
    onset = 0
    for n, p in enumerate(midi_filt):
        if p == p_prev:
            duration += 1
        else:
            # treat 0 as silence
            if p_prev > 0:
                # add note
                duration_sec = duration * hop / float(fs)
                # only add notes that are long enough
                if duration_sec >= minduration:
                    onset_sec = onset * hop / float(fs)
                    notes.append((onset_sec, duration_sec, p_prev))

            # start new note
            onset = n
            duration = 1
            p_prev = p

    # add last note
    if p_prev > 0:
        # add note
        duration_sec = duration * hop / float(fs)
        onset_sec = onset * hop / float(fs)
        notes.append((onset_sec, duration_sec, p_prev))

    return notes


def audio_to_midi_melodia(infile, outfile, smooth=0.25, minduration=0.1):
    # define analysis parameters
    fs = 44100
    hop = 128

    data, sr = librosa.load(infile)

    # extract melody using melodia vamp plugin
    print("Extracting melody f0 with MELODIA...")
    melody = vamp.collect(data, sr, "mtg-melodia:melodia")

    pitch = melody["vector"][1]

    # impute missing 0s to compensate for starting timestamp
    pitch = np.insert(pitch, 0, [0] * 8)

    # convert f0 to midi notes
    print("Converting Hz to MIDI notes...")
    midi_pitch = librosa.hz_to_midi(pitch)

    # segment sequence into individual midi notes
    notes = midi_to_notes(midi_pitch, fs, hop, smooth, minduration)

    # save note sequence to a midi file
    print("Saving MIDI to disk...")
    onset_env = librosa.onset.onset_strength(data, sr)
    tempo = librosa.beat.tempo(onset_envelope=onset_env, sr=sr)
    bpm = np.round(tempo[0])
    print(f"BPM: {bpm}")
    save_midi(outfile, notes, bpm)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("infile", help="Path to input audio file.")
    parser.add_argument("outfile", help="Path for saving output MIDI file.")
    args = parser.parse_args()

    audio_to_midi_melodia(args.infile, args.outfile)
