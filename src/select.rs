use crate::contour::PitchContour;
use crate::salience_peak::SaliencePeak;
use moving_avg::MovingAverage;
use stats::mean;
use std::collections::HashSet;

pub fn melody_pitch_mean(
    audio_len: usize,
    pitch_contours: &[PitchContour],
    smooth: bool,
) -> Vec<f32> {
    let mut pf: Vec<Vec<f32>> = std::iter::repeat(vec![]).take(audio_len).collect();
    for pitch_contour in pitch_contours {
        let s = pitch_contour.start;
        for (i, salience_peak) in pitch_contour.peaks.iter().enumerate() {
            pf[s + i].push(salience_peak.bin as f32);
        }
    }
    let out: Vec<f32> = pf
        .iter()
        .map(|x| mean(x.iter().copied()) as f32)
        .collect();
    if smooth {
        let mut ma = MovingAverage::<f32>::new(1722);
        out.iter().map(|x| ma.feed(*x)).collect()
    } else {
        out
    }
}

pub fn remove_octave_duplicates(pitch_contours: &mut Vec<PitchContour>, mpm: &[f32]) {
    pitch_contours.sort_by(|a, b| b.start.cmp(&a.start).then_with(|| b.end.cmp(&a.end)));
    let mut i = 0;
    let plen = pitch_contours.len();
    let mut deleter = HashSet::new();

    while i + 1 < plen {
        let curr = &pitch_contours[i];
        let mut j = i + 1;

        while j < plen {
            let cmpson = &pitch_contours[j];
            
            if cmpson.start > curr.end || deleter.contains(&i) {
                break;
            }
            if deleter.contains(&j) {
                j += 1;
                continue;
            }

            let rstart = std::cmp::max(cmpson.start, curr.start);
            let rend = std::cmp::min(cmpson.end, curr.end);
            let mut mean_overlaps = Vec::new();
            let mut mpm_overlaps = Vec::new();
            for (r, overlap) in mpm.iter().enumerate().take(rend + 1).skip(rstart) {
                mean_overlaps.push(
                    (curr.peaks[r - curr.start].bin as i32 - cmpson.peaks[r - rstart].bin as i32).abs(),
                );
                mpm_overlaps.push(*overlap);
            }

            let mean_overlap = mean(mean_overlaps.iter().copied()) as f32;
            if (115. ..=125.).contains(&mean_overlap) {
                let t1 = curr.total_salience();
                let t2 = cmpson.total_salience();

                if t1 < 0.5 * t2 {
                    deleter.insert(i);
                } else if t2 < 0.5 * t2 {
                    deleter.insert(j);
                } else {
                    let m1 = curr.mean_salience();
                    let m2 = cmpson.mean_salience();
                    let mpm_overlap = mean(mpm_overlaps.iter().copied()) as f32;
                    let diff1 = (m1 - mpm_overlap).abs();
                    let diff2 = (m2 - mpm_overlap).abs();

                    if diff1 > 60. && diff2 > 60. {
                        if t1 < t2 {
                            deleter.insert(i);
                        } else {
                            deleter.insert(j);
                        }
                    } else if diff2 < diff1 {
                        deleter.insert(i);
                    } else {
                        deleter.insert(j);
                    }
                }
            }

            j += 1;
        }

        i += 1;
    }

    let mut delete_indices: Vec<usize> = deleter.into_iter().collect();
    delete_indices.sort_unstable();
    for x in delete_indices.into_iter().rev() {
        pitch_contours.swap_remove(x);
    }
}

pub fn remove_pitch_outliers(pitch_contours: &mut Vec<PitchContour>, mpm: &[f32]) {
    let mut delete_indices: Vec<usize> = Vec::new();

    for (i, pitch_contour) in pitch_contours.iter_mut().enumerate() {
        if (mean(
            mpm[pitch_contour.start..pitch_contour.end + 1]
                .iter()
                .copied(),
        ) as f32
            - pitch_contour.mean_pitch())
        .abs()
            >= 120.
        {
            delete_indices.push(i);
        }
    }

    delete_indices.sort_unstable();
    for x in delete_indices.into_iter().rev() {
        pitch_contours.swap_remove(x);
    }
}

fn b2f(b: usize) -> f32 {
    (2.0_f32).powf(b as f32 / 120.) * 55.
}

pub fn select_final_freqs(audio_len: usize, pitch_contours: &mut Vec<PitchContour>) -> Vec<f32> {
    let mut spt: Vec<Vec<(SaliencePeak, f32)>> =
        std::iter::repeat(vec![]).take(audio_len).collect();
    for pitch_contour in pitch_contours {
        let s = pitch_contour.start;
        for (i, salience_peak) in pitch_contour.peaks.iter().enumerate() {
            spt[s + i].push((*salience_peak, pitch_contour.total_salience()));
        }
    }
    spt.into_iter()
        .map(|x| {
            let mut max_ele = f32::NEG_INFINITY;
            let mut max_bin = None;
            for (sp, t) in x {
                if t > max_ele {
                    max_ele = t;
                    max_bin = Some(sp);
                }
            }
            match max_bin {
                Some(x) => b2f(x.bin),
                None => 0.,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octave_dup() {
        let sp1 = SaliencePeak::new(1, 0.);
        let pc1 = PitchContour::new(sp1, 0);
        let sp2 = SaliencePeak::new(120, 1.);
        let pc2 = PitchContour::new(sp2, 0);
        let mpm: [f32; 1] = [120.];
        let mut pitch_contours = vec![pc1, pc2];

        remove_octave_duplicates(&mut pitch_contours, &mpm);

        assert!(pitch_contours.len() == 1);
    }
}
