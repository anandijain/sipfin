extern crate plotters;
use plotters::prelude::*;
use serde::Deserialize;
use std::env;

#[derive(Debug, PartialOrd, PartialEq, Clone, Default, Deserialize)]
struct Row {
    symb: String,
    t: f32,
    o: f32,
    h: f32,
    l: f32,
    c: f32,
    v: u64,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        panic!("arg 2: folder, arg 3: file_name. this looks in ../data/");
    }
    let input_fold = args[1].clone();
    let input_fn = args[2].clone();
    let data = get_data(&input_fold, &input_fn);
    let closes = data.iter().map(|x| x.c).collect::<Vec<f32>>();
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
            data[0].t..data[data.len() - 1].t,
            closes[0]..closes[closes.len() - 1],
        )?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(data.into_iter().map(|x| (x.t, x.c)), &RED))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn get_data(folder: &str, file_name: &str) -> Vec<Row> {
    let file_name_fmtd = format!("../data/{}/{}", folder, file_name);
    let mut rdr = csv::Reader::from_path(file_name_fmtd.clone()).expect(&file_name_fmtd);
    let iter = rdr.deserialize();
    let mut recs = vec![];
    for res in iter {
        if let Ok(r) = res {
            let rec: Row = r;
            recs.push(rec);
        }
    }
    recs
}
