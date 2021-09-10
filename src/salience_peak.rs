use crate::stft_peak::STFTFramePeaks;
use crate::util::peak_indices;
use stats::{mean, stddev};

#[derive(Copy, Clone, Debug)]
pub struct SaliencePeak {
    pub bin: usize,
    pub salience: f32,
}

impl SaliencePeak {
    pub fn new(b: usize, s: f32) -> Self {
        SaliencePeak {
            bin: b,
            salience: s,
        }
    }
}

impl PartialEq for SaliencePeak {
    fn eq(&self, other: &Self) -> bool {
        self.bin == other.bin && self.salience == other.salience
    }
}

pub struct SalienceFramePeaks {
    pub foreground: Vec<SaliencePeak>,
    pub background: Vec<SaliencePeak>,
}

impl SalienceFramePeaks {
    pub fn new(salience_peaks: &[SaliencePeak]) -> Self {
        let mut fg = Vec::new();
        let mut bg = Vec::new();
        let threshold = 0.9
            * salience_peaks
                .iter()
                .fold(f32::NEG_INFINITY, |a, b| a.max(b.salience));

        for salience_peak in salience_peaks {
            if salience_peak.salience < threshold {
                bg.push(*salience_peak);
            } else {
                fg.push(*salience_peak);
            }
        }

        let fg_sal: Vec<f32> = fg.iter().map(|x| x.salience).collect();
        let threshold =
            (mean(fg_sal.iter().copied()) - 0.9 * stddev(fg_sal.iter().copied())) as f32;
        
        for salience_peak in &fg {
            if salience_peak.salience < threshold {
                bg.push(*salience_peak);
            }
        }

        fg.retain(|&x| x.salience >= threshold);
        fg.sort_by(|a, b| b.salience.partial_cmp(&a.salience).unwrap());
        SalienceFramePeaks {
            foreground: fg,
            background: bg,
        }
    }

    pub fn extract_peak(&mut self, prev_cent: usize, gap_len: f32) -> Option<(SaliencePeak, f32)> {
        if gap_len <= 0. {
            return None;
        }

        let res;
        let mut i: Option<usize> = None;
        let mut j: Option<usize> = None;

        match self
            .foreground
            .iter()
            .find(|&x| (x.bin as i32 - prev_cent as i32).abs() <= 8)
        {
            Some(x) => {
                i = self.foreground.iter().position(|&y| y == *x);
                res = Some((*x, 100.));
            }
            None => {
                match self
                    .background
                    .iter()
                    .find(|&x| (x.bin as i32 - prev_cent as i32).abs() <= 8)
                {
                    Some(x) => {
                        j = self.background.iter().position(|&y| y == *x);
                        res = Some((*x, gap_len - 2.9));
                    }
                    None => res = None,
                }
            }
        }

        if let Some(x) = i {
            self.foreground.remove(x);
        }
        if let Some(x) = j {
            self.background.remove(x);
        }
        res
    }

    pub fn can_extract(&self) -> bool {
        !self.foreground.is_empty()
    }
}

pub fn salience_peaks(stft_frame_peaks: &[STFTFramePeaks]) -> Vec<SalienceFramePeaks> {
    stft_frame_peaks
        .iter()
        .map(|x| {
            let salience_peaks_raw = x.get_salience_bins();
            let ps = peak_indices(&salience_peaks_raw);
            let bins: Vec<SaliencePeak> = ps
                .iter()
                .map(|&x| SaliencePeak::new(x, salience_peaks_raw[x]))
                .collect();
            SalienceFramePeaks::new(&bins)
        })
        .collect()
}
