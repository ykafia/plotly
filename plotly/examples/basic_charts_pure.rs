use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::{Bar, NamedColor, PlotData as Plot, Rgb, Rgba, Scatter};

// Scatter Plots
fn simple_scatter_plot() {
    let n: usize = 100;
    let t: Vec<f64> = linspace(0., 10., n).collect();
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();

    let trace = Scatter::new(t, y).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    println!("{}", plot.to_json());
}

fn main() -> std::io::Result<()> {
    // Scatter Plots
    simple_scatter_plot();
    Ok(())
}
