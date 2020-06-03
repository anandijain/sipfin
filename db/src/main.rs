use noria::ControllerHandle;

use finox::nasdaq::info::InfoRoot;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db = ControllerHandle::from_zk("127.0.0.1:2181/yodel3")
        .await
        .unwrap();

    //println!("{:?}", db);
    //db.install_recipe(
    //    "
    //CREATE TABLE Quote(sid varchar(16), t varchar(32),  x varchar(16), PRIMARY KEY(sid));
    //CREATE TABLE Count(sid varchar(16), qid int);",
    //)
    //.await
    //.unwrap();

    let mut quotes = db.table("Quote").await.unwrap();
    let mut count = db.table("Count").await.unwrap();

    let (tickers, _) = finox::gen_secs("stocks");
    let urls = tickers[0..50]
        .iter()
        .map(|x| x.to_nasdaq_url("info"))
        .collect::<Vec<_>>();
    println!("{:#?}", urls);
    let recs = finox::fetch_one::<InfoRoot>(urls).await;
    println!("recs: {:#?}", recs);

    for (i, r) in recs.iter().enumerate() {
        match r {
            Some(row) => {
                quotes
                    .insert(vec![
                        row.data.symbol.to_string().into(),
                        row.data.primary_data.last_sale_price.clone().into(),
                        row.data.primary_data.last_trade_timestamp.clone().into(),
                    ])
                    .await
                    .unwrap();
                count
                    .insert(vec![row.data.symbol.clone().into(), i.into()])
                    .await
                    .unwrap();
            }
            _ => println!("missing rec"),
        }
    }

    db.extend_recipe(
        "
    QuoteCount: \
      SELECT Count.sid, COUNT(qid) AS counts \
      FROM Count GROUP BY Count.sid;
    QUERY ArticleWithVoteCount: \
      SELECT Quote.sid, t, x, VoteCount.counts AS counts\
      FROM Quote LEFT JOIN QuoteCount ON (Quote.sid = QuoteCount.sid) \
      WHERE Quote.sid = ?;",
    )
    .await
    .unwrap();
    let mut awvc = db.view("ArticleWithVoteCount").await.unwrap();
    println!("{:#?}", awvc.lookup(&["AAPL".into()], true).await);
    Ok(())
}
