use hyper_tls::HttpsConnector;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use hyper::Client;
use hyper::body::Buf;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    print!("{}[2J", 27 as char);
    println!("Aslaks VVB App\n\n");

    let url = "https://www.nordpoolgroup.com/api/marketdata/page/23?currency=NOK,NOK,EUR,EUR".parse().unwrap();
    let response = fetch_json(url).await?;
    let mut prices:Vec<Price> = Vec::new();

    response.data.rows.iter().filter(|row| match row.name.as_str() {
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
                        value: value
                  };

                  prices.push(price);

              })
        );


    let mut data:Vec<(f64, f64)> = Vec::new();
    // prices.sort_by(|a, b| a.value.cmp(&b.value));
    let mut i = 0.;
    prices.iter().for_each(|price| {
        data.push((i, price.value as f64 / 1000.0));
        i += 1.;
        // println!("{:0>2} - {:0>2}: {}", price.from.hour(), price.to.hour(), price.value)
    });

    let s1 = Plot::new(data).point_style(PointStyle::new().marker(PointMarker::Circle));
    let v = ContinuousView::new()
        .add(s1)
        .x_range(0., 24.)
        .y_range(0., 200.);

    println!("{}", Page::single(&v).dimensions(80, 30).to_text().unwrap());

    Ok(())
}

async fn fetch_json(url: hyper::Uri) -> Result<Root> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(url).await?;
    let body = hyper::body::aggregate(res).await?;

    Ok(serde_json::from_reader(body.reader())?)
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Price {
    from:DateTime<Utc>,
    to:DateTime<Utc>,
    value:i32
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