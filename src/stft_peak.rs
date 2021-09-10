use crate::util::peak_indices;
use num::complex::Complex;
use std::f32::consts::PI;
use stft::{WindowType, STFT};

pub struct STFTPeak {
    freq: f32,
    mag: f32,
}

impl STFTPeak {
    pub fn new(f: f32, m: f32) -> Self {
        STFTPeak { freq: f, mag: m }
    }
}

pub struct STFTFramePeaks {
    peaks: Vec<STFTPeak>,
    max_mag: f32,
}

impl Default for STFTFramePeaks {
    fn default() -> Self {
        STFTFramePeaks::new()
    }
}

impl STFTFramePeaks {
    pub fn new() -> Self {
        STFTFramePeaks {
            peaks: vec![],
            max_mag: f32::NEG_INFINITY,
        }
    }

    pub fn add(&mut self, f: f32, m: f32) {
        let p = STFTPeak::new(f, m);
        self.peaks.push(p);
        self.max_mag = f32::max(self.max_mag, m);
    }

    pub fn get_salience_bins(&self) -> [f32; 600] {
        let mut out = [0.; 600];

        for p in &self.peaks {
            if self.max_mag / p.mag >= 100. {
                continue;
            }

            let mut denom = 1.;

            while p.freq / denom >= 55. {
                let base = f2b(p.freq / denom) as i32;
                for b in base - 5..base + 6 {
                    let mut s = 0.;
                    for h in 1..21 {
                        for p in &self.peaks {
                            if self.max_mag / p.mag >= 100. {
                                continue;
                            }
                            let delta = get_delta(b as f32, h as f32, p.freq);
                            if !((-1. ..1.).contains(&delta)) {
                                continue;
                            }
                            s += get_harmonic_weight(delta, h as f32) * p.mag;
                        }
                    }
                    if (0..=599).contains(&b) {
                        out[b as usize] = s;
                    }
                }
                denom += 1.;
            }
        }

        out
    }
}

fn f2b(f: f32) -> f32 {
    assert!(f > 0.);
    (1200. * (f / 55.).log2() / 10.).round()
}

fn get_delta(b: f32, h: f32, f: f32) -> f32 {
    (f2b(f / h) - b).abs() / 10.
}

fn get_harmonic_weight(d: f32, h: f32) -> f32 {
    (d * PI / 2.).cos().powf(2.0) * (0.8_f32).powf(h - 1.)
}

fn hann_kernel(z: f32) -> f32 {
    ((PI * z).sin() / (PI * z)) / (2. * (1. - z.powf(2.)))
}

pub fn stft_peaks(audio: &[f32]) -> Vec<STFTFramePeaks> {
    let mut prev_audio = audio.to_vec();
    prev_audio.rotate_right(1);
    let mut stft = STFT::<f32>::new(WindowType::Hanning, 8192, 128);
    let mut prev_stft = STFT::<f32>::new(WindowType::Hanning, 8192, 128);
    let bands = stft.output_size();
    let mut mags: Vec<f32> = std::iter::repeat(0.).take(bands).collect();
    let mut phases: Vec<Complex<f32>> = std::iter::repeat(Complex::new(0., 0.))
        .take(bands)
        .collect();
    let mut prev_phases: Vec<Complex<f32>> = std::iter::repeat(Complex::new(0., 0.))
        .take(bands)
        .collect();
    let mut out: Vec<STFTFramePeaks> = Vec::new();

    stft.append_samples(audio);
    prev_stft.append_samples(&prev_audio[..]);

    while stft.contains_enough_to_compute() {
        stft.compute_column(&mut mags[..]);
        mags = mags.iter().map(|x| (x * 2. / 1023.5).abs()).collect();
        stft.compute_complex_column(&mut phases[..]);
        prev_stft.compute_complex_column(&mut prev_phases[..]);
        let ps = peak_indices(&mags);
        let mut row = STFTFramePeaks::default();

        for peak in ps {
            let mut offset =
                phases[peak].arg() - prev_phases[peak].arg() - 2. * PI * peak as f32 / 8192.;
            offset = (offset + PI) % (2.0 * PI) - PI;
            offset *= 8192. / (2. * PI);
            let ins_freq = (peak as f32 + offset) * 44100. / 8192.;
            if !(55. ..=1760.).contains(&ins_freq) {
                continue;
            }
            let ins_mag = mags[peak] / (2. * hann_kernel(offset / 4.));
            row.add(ins_freq, ins_mag);
        }

        stft.move_to_next_column();
        prev_stft.move_to_next_column();
        out.push(row);
    }

    out
}
