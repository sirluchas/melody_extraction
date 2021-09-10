use crate::salience_peak::{SalienceFramePeaks, SaliencePeak};
use stats::{mean, stddev};

pub struct PitchContour {
    pub peaks: Vec<SaliencePeak>,
    pub start: usize,
    pub end: usize,
    multiplier: f32,
}

impl PitchContour {
    pub fn new(peak: SaliencePeak, i: usize) -> Self {
        PitchContour {
            peaks: vec![peak],
            start: i,
            end: i,
            multiplier: 1.,
        }
    }

    pub fn add(&mut self, peak: SaliencePeak, i: usize) {
        if i < self.start {
            self.peaks.insert(0, peak);
            self.start = i;
        } else if i > self.end {
            self.peaks.push(peak);
            self.end = i;
        } else {
            panic!("New index doesnt change pitch contour start or end");
        }
    }

    pub fn mean_salience(&self) -> f32 {
        mean(self.peaks.iter().map(|x| x.salience)) as f32
    }

    pub fn total_salience(&self) -> f32 {
        self.peaks.iter().map(|x| x.salience).sum::<f32>() * self.multiplier
    }

    pub fn mean_pitch(&self) -> f32 {
        mean(self.peaks.iter().map(|x| x.bin)) as f32
    }

    pub fn pitch_deviance(&self) -> f32 {
        stddev(self.peaks.iter().map(|x| x.bin)) as f32
    }

    pub fn set_multiplier(&mut self) {
        if self.pitch_deviance() > 4. {
            self.multiplier = 1.5;
        }
    }
}

pub fn pitch_contours(salience_frame_peaks: &mut [SalienceFramePeaks]) -> Vec<PitchContour> {
    let mut out = Vec::new();
    let mut sals = Vec::new();
    let mut i = 0;

    while i < salience_frame_peaks.len() {
        while salience_frame_peaks[i].can_extract() {
            let curr = &mut salience_frame_peaks[i];
            let mut pc = PitchContour::new(curr.foreground[0], i);
            curr.foreground.remove(0);

            for dir in [usize::checked_add, usize::checked_sub].iter() {
                let mut j = i;
                match dir(j, 1) {
                    Some(x) => j = x,
                    None => break,
                }
                let mut gap_len: f32 = 100.;
                let mut prev_cent = pc.peaks[0].bin;

                while j < salience_frame_peaks.len() && gap_len > 0. {
                    match salience_frame_peaks[j].extract_peak(prev_cent, gap_len) {
                        Some(tup) => {
                            let (peak, pcent) = tup;
                            pc.add(peak, j);
                            prev_cent = peak.bin;
                            gap_len = pcent;
                        }
                        None => break,
                    }
                    match dir(j, 1) {
                        Some(x) => j = x,
                        None => break,
                    }
                }
            }

            sals.push(pc.mean_salience());
            pc.set_multiplier();
            out.push(pc);
        }
        i += 1;
    }

    let threshold = (mean(sals.iter().copied()) - 0.9 * stddev(sals.iter().copied())) as f32;
    out.retain(|x| x.mean_salience() >= threshold);

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::salience_peak::{SalienceFramePeaks, SaliencePeak};

    #[test]
    fn test_gap_len() {
        let mut sfp = Vec::new();
        let mut ans = Vec::new();

        let mut curr = SalienceFramePeaks::new(&Vec::new());
        curr.foreground = vec![SaliencePeak::new(22, 1.)];
        curr.background = vec![];
        sfp.push(curr);
        ans.push(SaliencePeak::new(22, 1.));

        for _ in 1..37 {
            let mut curr = SalienceFramePeaks::new(&Vec::new());
            curr.foreground = vec![SaliencePeak::new(9, 1.)];
            curr.background = vec![SaliencePeak::new(20, 1.)];
            sfp.push(curr);
            ans.push(SaliencePeak::new(20, 1.));
        }

        let pc = pitch_contours(&mut sfp);
        assert!(pc[0].peaks.len() == 36);
        assert!(
            pc[0]
                .peaks
                .iter()
                .zip(&ans)
                .filter(|&(x, y)| x == y)
                .count()
                == 36
        );
    }

    #[test]
    fn test_backwards() {
        let mut sfp = Vec::new();
        let mut ans = Vec::new();

        let mut curr = SalienceFramePeaks::new(&Vec::new());
        curr.foreground = vec![SaliencePeak::new(22, 1.)];
        curr.background = vec![SaliencePeak::new(13, 1.)];
        sfp.push(curr);
        ans.push(SaliencePeak::new(13, 1.));

        let mut curr = SalienceFramePeaks::new(&Vec::new());
        curr.foreground = vec![SaliencePeak::new(12, 1.)];
        curr.background = vec![];
        sfp.push(curr);
        ans.push(SaliencePeak::new(12, 1.));

        let pc = pitch_contours(&mut sfp);
        assert!(pc.len() == 2);
        assert!(pc[1].peaks.len() == 2);
        assert!(
            pc[1]
                .peaks
                .iter()
                .zip(&ans)
                .filter(|&(x, y)| x == y)
                .count()
                == 2
        );
    }
}
