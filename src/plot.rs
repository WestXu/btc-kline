use chrono::prelude::*;
use itertools::Itertools;
use plotly::configuration::DisplayModeBar;
use plotly::layout::{Axis, RangeSlider};
use plotly::{Candlestick, Configuration, Layout, Plot};

use crate::fitting::best_fit;
use crate::prices::Kline;

pub async fn plot(kline_data: &Vec<Kline>, dark: bool) -> Plot {
    let (open, close, high, low, x): (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<String>) =
        kline_data
            .iter()
            .map(|kline| {
                (
                    kline.open.parse::<f64>().unwrap(),
                    kline.close.parse::<f64>().unwrap(),
                    kline.high.parse::<f64>().unwrap(),
                    kline.low.parse::<f64>().unwrap(),
                    format!(
                        "{}",
                        Local
                            .timestamp_millis_opt(kline.open_time as i64)
                            .unwrap()
                            .format("%Y-%m-%d %H:%M:%S")
                    ),
                )
            })
            .multiunzip();

    let latest = *close.last().unwrap();
    let first = *open.first().unwrap();
    let change = latest / first - 1.0;

    let fit_trace = best_fit(&close);

    let trace = Candlestick::new(x.clone(), open, high, low, close).name("Kline");
    let fit_trace = plotly::Scatter::new(x, fit_trace)
        .line(plotly::common::Line::new().dash(plotly::common::DashType::Dash))
        .name("PolyFit");

    let mut plot = Plot::new();
    plot.add_trace(Box::new(trace));
    plot.add_trace(fit_trace);
    let layout = Layout::new()
        .title(
            format!(
                "<b>BTC   {:.2}   {}{:.2}%</b>",
                latest,
                if change.is_sign_positive() { "+" } else { "" },
                change * 100.0
            )
            .as_str()
            .into(),
        )
        .hover_mode(plotly::layout::HoverMode::X)
        .show_legend(false)
        .template(if dark {
            plotly::layout::themes::PLOTLY_DARK.clone()
        } else {
            plotly::layout::themes::DEFAULT.clone()
        })
        .x_axis(Axis::new().range_slider(RangeSlider::new().visible(false)))
        .y_axis(Axis::new().tick_format(".2f"));
    plot.set_layout(layout);
    plot.set_configuration(
        Configuration::new()
            .display_mode_bar(DisplayModeBar::False)
            .responsive(true)
            .fill_frame(true),
    );

    // plot.use_local_plotly();
    plot
}
