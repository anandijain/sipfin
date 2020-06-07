extern crate reqwest;

pub const ROOT: &str = "https://paper-api.alpaca.markets";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let req_ep = "account";
    let post_ep = "orders";
    let pos_ep = "positions";
    let clock_url = format!("{}/v2/clock", ROOT);
    let request_url = format!("{}/v2/{}", ROOT, req_ep);
    let post_url = format!("{}/v2/{}", ROOT, post_ep);
    let positions_url = format!("{}/v2/{}", ROOT, pos_ep);
    let history_url = format!("{}/v2/account/portfolio/history", ROOT);

    let headers = apca::get_headermap();
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let clock_res = client
        .get(&clock_url)
        .send()
        .await?
        .json::<apca::Clock>()
        .await?;
    println!("{:#?}", clock_res);
    //assert!(clock_res.is_open, "market is closed, not sending order");

    let res = client
        .get(&request_url)
        .send()
        .await?
        .json::<apca::ApcaAccount>()
        .await?;

    println!("{:#?}", res);

    let buy = apca::MarketOrder {
        symbol: "AAPL".to_string(),
        qty: 1,
        side: "buy".to_string(),
        type_field: "market".to_string(),
        time_in_force: "day".to_string(),
    };

    let sell = apca::MarketOrder {
        symbol: "OXY".to_string(),
        qty: 1,
        side: "sell".to_string(),
        type_field: "market".to_string(),
        time_in_force: "day".to_string(),
    };

    let bracket_buy = apca::BracketOrder {
        side: "buy".to_string(),
        symbol: "OXY".to_string(),
        type_field: "stop".to_string(),
        qty: 100.to_string(),
        time_in_force: "gtc".to_string(),
        order_class: "bracket".to_string(),
        take_profit: apca::TakeProfit {
            limit_price: 26.to_string(),
        },
        stop_loss: apca::StopLoss {
            stop_price: 23.to_string(),
            limit_price: 22.to_string(),
        },
    };

    //let bracket_buy = apca::BracketOrder {
    //    symbol: "OXY".to_string(),
    //    qty: 1,
    //    side: "sell".to_string(),
    //    type_field: "market".to_string(),
    //    time_in_force: "day".to_string(),
    //};

    //let orders = vec![buy, sell];
    println!("orders: {:#?}", bracket_buy,); //orders);

    let order_res = client
        .post(&post_url)
        .json(&bracket_buy)
        .send()
        .await?
        //.json::<apca::ApcaOrder>()
        .json::<serde_json::Value>()
        .await?;

    println!("order res {:#?}", order_res);
    let positions = client
        .get(&positions_url)
        .send()
        .await?
        .json::<Vec<apca::ApcaPosition>>()
        .await?;

    println!("{:#?}", positions);

    let history = client
        .get(&history_url)
        .send()
        .await?
        .json::<apca::PortfolioHistory>()
        .await?;

    println!("{:#?}", history);

    Ok(())
}
