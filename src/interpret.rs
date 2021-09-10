use plotters::prelude::*;

pub fn plot_results(
    data: &[f32],
    audio_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(audio_path, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..data.len() as f32 * 128. / 44100., 0f32..2000f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            data.iter()
                .enumerate()
                .map(|(x, y)| (x as f32 * 128. / 44100., *y)),
            &RED,
        ))?
        .label("melody contours")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
