use hyper_tls::HttpsConnector;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use hyper::Client;
use hyper::body::Buf;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    match Local::now().hour() {
        0..=12 => println!("Before 13:00"),
        _ => println!("After 13:00")
    }

    // let prices = get_prices(Some(Utc::today())).await;
    let prices = get_prices(None).await;
    let prices = approve_lowest_prices(prices.unwrap(), 8);

    println!("{:#?}", prices);
    // get_prices(None).await;

    loop {}
}

// #[tokio::main]
// async fn main() {
//     println!("Aslaks VVB App\n\n");
//
//     let sched = JobScheduler::new().unwrap();
//
//     #[cfg(feature = "signal")]
//     sched.shutdown_on_ctrl_c();
//
//     let get_prices_job = sched.add(Job::new_async("1/10 * * * * *", |_uuid, _l| {
//         Box::pin(async move {
//             let prices = get_prices().await;
//
//             match prices {
//                 Err(e) => println!("{:?}", e),
//                 _ => print_graph(prices.unwrap()).await
//             }
//         })
//     }).unwrap());
//
//     if get_prices_job.is_err() {
//         println!("Error starting get data job");
//         return;
//     }
//
//     let start = sched.start();
//
//     if start.is_err() {
//         println!("Error starting scheduler");
//         return;
//     }
//
//     loop {}
// }

fn approve_lowest_prices(prices: Vec<Price>, count: usize) -> Vec<Price> {
    let mut approved_prices = prices.clone();
    approved_prices.sort_by(|a, b| a.value.cmp(&b.value));

    for i in 0..count {
        approved_prices[i].approved = true;
    }

    approved_prices.sort_by(|a, b| a.from.cmp(&b.from));
    approved_prices
}

async fn print_graph(prices: Vec<Price>) {
    // Clear screen
    print!("{}[2J", 27 as char);

    let mut prices_clone = prices.clone();
    prices_clone.sort_by(|a, b| a.value.cmp(&b.value));
    let lowest = prices_clone[0].value as f64 / 1000.0;
    prices_clone.sort_by(|a, b| b.value.cmp(&a.value));
    let highest = prices_clone[0].value as f64 / 1000.0;
    println!("{} {}", lowest, highest);

    let mut data:Vec<(f64, f64)> = Vec::new();
    let mut i = 0.;
    prices.iter().for_each(|price| {
        data.push((i, price.value as f64 / 1000.0));
        i += 1.;
        // println!("{:0>2} - {:0>2}: {}", price.from.hour(), price.to.hour(), price.value)
        println!("{:#?} - {:#?}: {}", price.from, price.to, price.value)
    });

    let s1 = Plot::new(data).point_style(PointStyle::new().marker(PointMarker::Circle));
    let v = ContinuousView::new()
        .add(s1)
        .x_range(0., 24.)
        .y_range(lowest - 10., highest + 10.);

    println!("{}", Page::single(&v).dimensions(80, 20).to_text().unwrap());
}

async fn get_prices(date: Option<Date<Utc>>) -> Result<Vec<Price>> {
    let mut url = String::from("https://www.nordpoolgroup.com/api/marketdata/page/23?currency=,,,NOK");

    match date {
        Some(d) => url = format!("{}&endDate={}", url, d.format("%d-%m-%Y")),
        None => ()
    }

    let response = fetch_json(url.parse().unwrap()).await?;
    let mut prices:Vec<Price> = Vec::new();

    response.data.rows.iter()
        .filter(|row| match row.name.as_str() {
            "00&nbsp;-&nbsp;01" => true,
            "01&nbsp;-&nbsp;02" => true,
            "02&nbsp;-&nbsp;03" => true,
            "03&nbsp;-&nbsp;04" => true,
            "04&nbsp;-&nbsp;05" => true,
            "05&nbsp;-&nbsp;05" => true,
            "06&nbsp;-&nbsp;07" => true,
            "07&nbsp;-&nbsp;08" => true,
            "08&nbsp;-&nbsp;09" => true,
            "09&nbsp;-&nbsp;10" => true,
            "10&nbsp;-&nbsp;11" => true,
            "11&nbsp;-&nbsp;12" => true,
            "12&nbsp;-&nbsp;13" => true,
            "13&nbsp;-&nbsp;14" => true,
            "14&nbsp;-&nbsp;15" => true,
            "15&nbsp;-&nbsp;16" => true,
            "16&nbsp;-&nbsp;17" => true,
            "17&nbsp;-&nbsp;18" => true,
            "18&nbsp;-&nbsp;19" => true,
            "19&nbsp;-&nbsp;20" => true,
            "20&nbsp;-&nbsp;21" => true,
            "21&nbsp;-&nbsp;22" => true,
            "22&nbsp;-&nbsp;23" => true,
            "23&nbsp;-&nbsp;00" => true,
            _ => false
        })
        .for_each(|row| row.columns.iter()
            .filter(|column| column.name.eq("Oslo"))
            .for_each(|column| {
                let start_time = format!("{}z", row.start_time);
                let end_time = format!("{}z", row.end_time);
                let value:i32 = column.value.trim()
                    .replace(' ', "")
                    .replace(',', "")
                    .parse()
                    .unwrap();

                let price = Price {
                    from: start_time.parse::<DateTime<Utc>>().unwrap(),
                    to: end_time.parse::<DateTime<Utc>>().unwrap(),
                    value: value,
                    approved: false
                };

                prices.push(price);
            })
        );

    Ok(prices)
}

async fn fetch_json(url: hyper::Uri) -> Result<Root> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(url).await?;
    let body = hyper::body::aggregate(res).await?;

    Ok(serde_json::from_reader(body.reader())?)
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Price {
    from:DateTime<Utc>,
    to:DateTime<Utc>,
    value:i32,
    approved:bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    #[serde(rename="cacheKey")]
    cache_key:String,
    #[serde(rename="pageId")]
    page_id:u32,
    currency:String,
    data:Data
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename="Rows")]
    rows:Vec<Row>
}

#[derive(Serialize, Deserialize, Debug)]
struct Column {
    #[serde(rename="Name")]
    name:String,
    #[serde(rename="Value")]
    value:String
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    #[serde(rename="Name")]
    name:String,
    #[serde(rename="StartTime")]
    start_time:String,
    #[serde(rename="EndTime")]
    end_time:String,
    #[serde(rename="Columns")]
    columns:Vec<Column>
}
