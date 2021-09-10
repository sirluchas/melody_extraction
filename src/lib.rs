pub mod filter;
pub use filter::equal_loudness;

pub mod contour;
pub use contour::pitch_contours;
pub mod salience_peak;
pub use salience_peak::salience_peaks;
pub mod select;
pub use select::{
    melody_pitch_mean, remove_octave_duplicates, remove_pitch_outliers, select_final_freqs,
};
pub mod stft_peak;
pub use stft_peak::stft_peaks;

pub mod interpret;
pub use interpret::plot_results;
pub mod util;
pub use util::*;

pub fn salamon(audio_signal: &[f32]) -> Vec<f32> {
    let audio_signal = equal_loudness(audio_signal);
    let stft_frame_peaks = stft_peaks(&audio_signal);
    let mut salience_peaks = salience_peaks(&stft_frame_peaks);
    let audio_len = salience_peaks.len();
    let mut pitch_contours = pitch_contours(&mut salience_peaks);
    let mut mpm = melody_pitch_mean(audio_len, &pitch_contours, true);

    for _ in 0..3 {
        remove_octave_duplicates(&mut pitch_contours, &mpm);
        mpm = melody_pitch_mean(audio_len, &pitch_contours, false);
        remove_pitch_outliers(&mut pitch_contours, &mpm);
        mpm = melody_pitch_mean(audio_len, &pitch_contours, false);
    }

    select_final_freqs(audio_len, &mut pitch_contours)
}
