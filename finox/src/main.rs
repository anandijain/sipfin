extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;



mod getters;
mod types;
mod utils;

fn main() -> Result<(), reqwest::Error> {
    currencies()
}

fn sp500() -> Result<(), reqwest::Error> {
    let write_fn = "./data/sp500.csv".to_string();
    let symbs = utils::read_tickers("./data/sp500tickers.txt");
    getters::get_csv(
        getters::company_to_rec,
        &utils::stock_header,
        symbs.to_vec(),
        write_fn,
        1000,
    );
    Ok(())
}

// fn sp500_headlines() -> Result<(), reqwest::Error> {

//     let write_fn = "./data/sp500_headlines.csv".to_string();
//     let symbs = utils::read_tickers("./data/sp500tickers.txt");
//     // let index = symbs.iter().position(|r| r.to_string() == "COF").unwrap();
//     // let todo_symbs = &symbs[index..symbs.len()];
//     getters::get_csv(
//         getters::headlines_to_rec,
//         &utils::headlines_header,
//         symbs.to_vec(),
//         write_fn,
//         1000,
//     );
//     Ok(())
// }

fn currencies() -> Result<(), reqwest::Error> {
    //"VEF",

    for s in utils::currency_symbols.iter() {
        // getters::get_csv(
            //     getters::currency_records,
            //     vec!["date_time".to_string(), format!("USD{}", s.to_string())],
            //     vec![s.to_string()],
            //     write_fn,
            //     1000,
            // );
            if let Ok(curs) = getters::get_currency(s.to_string()) {
                let write_fn = format!("./data/test_USD{}.csv", s.to_string());
                if let Ok(recs) = types::Intraday::to_records(&curs[0]){
                    utils::writerecs(write_fn, &["date_time", &curs[0].ticker.to_string()], recs);
            
            }
        }
    }
    Ok(())
}
