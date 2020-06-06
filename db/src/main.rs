use finox::nasdaq::realtime::RealtimeRoot;
use noria::prelude::*;
use std::{error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db = ControllerHandle::from_zk("127.0.0.1:2181/isit8")
        .await
        .unwrap();

    db.install_recipe(
        "
    CREATE TABLE Rt(sid varchar(16), t varchar(32), x float, v int, PRIMARY KEY(sid));
    CREATE TABLE Quote(sid varchar(16), qid varchar(32));",
    )
    .await
    .unwrap();

    let mut quotes = db.table("Rt").await.unwrap();
    let mut count = db.table("Quote").await.unwrap();
    println!("{:#?} ", quotes.schema());
    println!("{:#?} ", count.schema());

    let (tickers, _) = finox::gen_secs("stocks");
    let urls = tickers[1..5]
        .iter()
        .map(|x| x.to_nasdaq_rt_url())
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect();

    let recs = finox::fetch::<RealtimeRoot>(urls)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<Vec<String>>>();
    let mut noria_recs = vec![];

    for rec in recs.iter() {
        let noria_rec: Vec<noria::DataType> = rec.iter().map(|x| x.to_string().into()).collect();
        println!("{:#?} ", noria_rec);
        noria_recs.push(noria_rec);
    }
    quotes.perform_all(noria_recs).await.unwrap();
    //count
    //    .insert(vec![r[0].clone().into(), format!("{}{}", i, j).into()])
    //    .await
    //    .unwrap();

    println!("Finished writing! Let's wait for things to propagate...");
    tokio::time::delay_for(Duration::from_millis(2000)).await;

    db.extend_recipe(
        "
    QuoteCount: \
        SELECT Quote.sid, COUNT(qid) as counts \
        FROM Quote GROUP BY Quote.sid;
    QUERY Quotes: \
        SELECT Rt.sid, t, x, v, QuoteCount.counts AS counts \
        FROM Rt LEFT JOIN QuoteCount ON (Rt.sid = QuoteCount.sid) \
        WHERE Rt.sid = ?;
    ",
    )
    .await
    .unwrap();
    let mut awvc = db.view("Quotes").await.unwrap();
    //imdumb
    let ticks2 = finox::roses::read_tickers("../ref_data/tickers_stocks.txt");
    //for tic in ticks2.iter() {
    let article = awvc.lookup(&["aa".into()], true).await.unwrap();
    println!("{:#?} ", article);
    //}
    Ok(())
}
