use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use crate::nordpool::Price;

pub fn print_graph(prices: Vec<Price>) {
    // Clear screen
    print!("{}[2J", 27 as char);

    let mut prices_clone = prices.clone();
    prices_clone.sort_by(|a, b| a.value.cmp(&b.value));
    let lowest = prices_clone[0].value as f64 / 1000.0;
    prices_clone.sort_by(|a, b| b.value.cmp(&a.value));
    let highest = prices_clone[0].value as f64 / 1000.0;

    let mut data:Vec<(f64, f64)> = Vec::new();
    let mut i = 0.;
    prices.iter().for_each(|price| {
        data.push((i, price.value as f64 / 1000.0));
        i += 1.;
    });

    let s1 = Plot::new(data).point_style(PointStyle::new().marker(PointMarker::Circle));
    let v = ContinuousView::new()
        .add(s1)
        .x_range(0., 24.)
        .y_range(lowest - 10., highest + 10.);

    println!("{}", Page::single(&v).dimensions(80, 20).to_text().unwrap());
}
