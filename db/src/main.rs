use noria::ControllerHandle;

use finox::nasdaq::realtime::RealtimeRoot;
use std::{error::Error, time::Duration};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db = ControllerHandle::from_zk("127.0.0.1:2181/isit4")
        .await
        .unwrap();

    db.install_recipe(
        "
    CREATE TABLE Rt(sid varchar(16), t varchar(32), x varchar(16), v varchar(16), PRIMARY KEY(sid));
    CREATE TABLE Count(sid varchar(16), qid int);",
    )
    .await
    .unwrap();

    let mut quotes = db.table("Rt").await.unwrap();
    let mut count = db.table("Count").await.unwrap();

    let (tickers, _) = finox::gen_secs("stocks");
    let urls = tickers[1..50]
        .iter()
        .map(|x| x.to_nasdaq_rt_url())
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect();

    //println!("{:#?}", urls);
    let recs = finox::fetch::<RealtimeRoot>(urls).await;
    println!("recs: {:#?}", recs);

    for (_, rs) in recs.iter().enumerate() {
        for (_, r) in rs.iter().enumerate() {
            quotes
                .insert(vec![
                    r[0].clone().into(),
                    r[1].clone().into(),
                    r[2].clone().into(),
                    r[3].clone().into(),
                ]) //.iter().map(|x| x.into()).collect::<Vec<_>>())
                .await
                .unwrap();
        }
    }
    println!("Finished writing! Let's wait for things to propagate...");
    tokio::time::delay_for(Duration::from_millis(2000)).await;

    db.extend_recipe(
    "
    QUERY Quotes: \
        SELECT Rt.sid, t, x, v from Rt;
    "
    //    "
    //QuoteCount: \
    //  SELECT Count.sid, COUNT(qid) AS counts \
    //  FROM Count GROUP BY Count.sid;
    //QUERY ArticleWithQuoteCount: \
    //  SELECT Quote.sid, t, x, QuoteCount.counts AS counts\
    //  FROM Quote LEFT JOIN QuoteCount ON (Quote.sid = QuoteCount.sid) \
    //  WHERE Quote.sid = ?;",
    )
    .await
    .unwrap();
    let mut awvc = db.view("Quotes").await.unwrap();
    let article = awvc.lookup(&["aapl".into()], true).await.unwrap();
    println!("{:#?} {:#?} ", awvc.len().await, article);
    Ok(())
}
