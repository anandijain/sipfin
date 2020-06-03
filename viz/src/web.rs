extern crate tokio;
use async_std::task;
use finox::Security;
use tide;

fn main() -> tide::Result<()> {
    task::block_on(async {
        let mut app = tide::new();
        //serde_json::Value::from(infos);
        //for info in infos.iter() {
        //    println!("{:#?}", json!(info));
        //}

        //let info_str = format!("{:#?}", infos).to_string();
        //println!("{:#?}", info_str);
        //    let infos = finox::fetch_one::<finox::nasdaq::info::InfoRoot>(urls)
        //                .await
        //                .into_iter()
        //                .flatten()
        //                .collect::<Vec<finox::nasdaq::info::InfoRoot>>();
        //
        //            let to_serve = serde_json::to_string(&infos.clone()).expect("fuck");

        app.at("/infos").get(|_| async {
            let filepath = "../ref_data/tickers_stocks.txt";
            let urls = roses::read_tickers(filepath)
                .iter()
                .map(|x| Security::Stock(x.to_string()).to_nasdaq_url("info"))
                .collect::<Vec<String>>();

            Ok(serde_json::to_string(&urls.clone())?)
        });
        app.listen("127.0.0.1:8080").await?;
        Ok(())
    })
}
