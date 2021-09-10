use digital_filter::DigitalFilter;

pub fn equal_loudness(audio: &[f32]) -> Vec<f32> {
    let yulewalk_coeffs = [
        0.054186564,
        0.00836867,
        -0.0013337823,
        0.0009958048,
        -0.00088107685,
        -0.0025471316,
        -0.0037880547,
        -0.0036973201,
        -0.0010968408,
        -0.008982358,
        -0.014279355,
        0.,
    ];
    let mut yulewalk_filter = DigitalFilter::new(yulewalk_coeffs);
    let butter_coeffs = [0.98500174, 1.0001142, 1.0152266, 0.];
    let mut butter_filter = DigitalFilter::new(butter_coeffs);
    audio
        .iter()
        .map(|x| butter_filter.filter(yulewalk_filter.filter(*x)))
        .collect()
}
