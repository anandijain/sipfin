extern crate plotters;
use chrono::DateTime;
use plotters::prelude::*;
use serde::Deserialize;
use std::env;
#[derive(Debug, PartialOrd, PartialEq, Clone, Default, Deserialize)]
struct YfRow {
    symb: String,
    t: f32,
    o: f32,
    h: f32,
    l: f32,
    c: f32,
    v: u64,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Default, Deserialize)]
struct RtRow {
    symbol: String,
    t: String,
    x: f32,
    v: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        panic!("arg 2: folder, arg 3: file_name. this looks in ../data/");
    }
    let input_fold = args[1].clone();
    let input_fn = args[2].clone();
    //let data: Vec<YfRow> = get_data(&input_fold, &input_fn);
    let data: Vec<RtRow> = get_data(&input_fold, &input_fn);
    let xs = data.iter().map(|x| x.x).collect::<Vec<f32>>();
    let ts = data
        .iter()
        .map(|x| DateTime::parse_from_rfc3339(&x.t).unwrap().timestamp())
        .collect::<Vec<i64>>();
    //let closes = data.iter().map(|x| x.c).collect::<Vec<f32>>();
    //let min =
    let plot_fn = format!("../data/viz/{}.png", input_fn);
    let root = BitMapBackend::new(&plot_fn, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(input_fn, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(
            ts[0]..ts[data.len() - 1],
            //closes[0]..closes[closes.len() - 1],
            xs[0]..xs[xs.len() - 1],
        )?;

    let to_plot = data
        .iter()
        .map(|x| (DateTime::parse_from_rfc3339(&x.t).unwrap().timestamp(), x.x))
        .into_iter();

    chart.configure_mesh().draw()?;
    let series = LineSeries::new(to_plot, &RED);
    chart.draw_series(series);

    //chart
    //    .draw_series(data.iter().map(|x| {
    //        CandleStick::new(
    //            //DateTime::parse_from_rfc3339(x.t).unwrap_or(panic!("fuck dates")),
    //            x.t, x.o, x.h, x.l, x.c, &GREEN, &RED, 15,
    //        )
    //    }))?
    //    .label("y = x^2")
    //    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

pub fn get_data<'a, T: ?Sized>(folder: &str, file_name: &str) -> Vec<T>
where
    for<'de> T: serde::Deserialize<'de> + 'a,
{
    let file_name_fmtd = format!("../data/{}/{}", folder, file_name);
    let mut rdr = csv::Reader::from_path(file_name_fmtd.clone()).expect(&file_name_fmtd);
    let iter = rdr.deserialize();
    let mut recs = vec![];
    for res in iter {
        if let Ok(r) = res {
            let rec: T = r;
            recs.push(rec);
        }
    }
    recs
}
