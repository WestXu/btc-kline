use chrono::prelude::*;
use itertools::Itertools;
use plotly::configuration::DisplayModeBar;
use plotly::layout::{Axis, RangeSlider};
use plotly::{Candlestick, Configuration, Layout, Plot};

pub async fn plot() -> Plot {
    let kline_data = super::prices::get_kline().await;

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
                            .timestamp_millis_opt(kline.close_time as i64)
                            .unwrap()
                            .format("%Y-%m-%d %H:%M:%S")
                    ),
                )
            })
            .multiunzip();

    let latest = *close.last().unwrap();
    let first = *open.first().unwrap();
    let change = latest / first - 1.0;

    let trace = Candlestick::new(x, open, high, low, close);

    let mut plot = Plot::new();
    plot.add_trace(trace);
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
