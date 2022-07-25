use hyper_tls::HttpsConnector;
use hyper::Client;
use hyper::body::Buf;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync>>;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Price {
    from: DateTime<Local>,
    to: DateTime<Local>,
    pub value: i32,
    pub approved: bool,
    pub active: bool
}

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no price found")
    }
}

impl error::Error for EmptyVec {}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    #[serde(rename="cacheKey")]
    cache_key: String,
    #[serde(rename="pageId")]
    page_id: u32,
    currency: String,
    data: Data
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename="Rows")]
    rows: Vec<Row>
}

#[derive(Serialize, Deserialize, Debug)]
struct Column {
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="Value")]
    value: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="StartTime")]
    start_time: String,
    #[serde(rename="EndTime")]
    end_time: String,
    #[serde(rename="Columns")]
    columns: Vec<Column>
}

pub async fn get_prices(date: Option<Date<Local>>) -> Result<Vec<Price>> {
    let mut url = String::from("https://www.nordpoolgroup.com/api/marketdata/page/23?currency=,,,NOK");

    // TODO: "if let" here?
    match date {
        Some(d) => url = format!("{}&endDate={}", url, d.format("%d-%m-%Y")),
        None => ()
    }

    let response = fetch_json(url.parse()?).await?;
    let mut prices:Vec<Price> = Vec::new();

    response.data.rows.iter()
        .filter(|row| match row.name.as_str() {
            "00&nbsp;-&nbsp;01" => true,
            "01&nbsp;-&nbsp;02" => true,
            "02&nbsp;-&nbsp;03" => true,
            "03&nbsp;-&nbsp;04" => true,
            "04&nbsp;-&nbsp;05" => true,
            "05&nbsp;-&nbsp;06" => true,
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
                let start_time = row.start_time.parse().unwrap();
                let end_time = row.end_time.parse().unwrap();
                let value:i32 = column.value.trim()
                    .replace(' ', "")
                    .replace(',', "")
                    .parse()
                    .unwrap();

                let from = Local.from_local_datetime(&start_time).unwrap();
                let to = Local.from_local_datetime(&end_time).unwrap();
                let now = Local::now();

                let mut active = false;
                if from.year() == now.year() &&
                    from.month() == now.month() &&
                    from.day() == now.day() &&
                    from.hour() == now.hour() {
                        active = true;
                    }

                let price = Price {
                    from,
                    to,
                    value,
                    approved: false,
                    active
                };

                prices.push(price);
            })
        );

    Ok(prices)
}

pub fn get_current_price(prices: &Vec<Price>) -> Result<&Price> {
    let now = Local::now();

    prices.into_iter().find(|&price| {
        price.from.year() == now.year() &&
        price.from.month() == now.month() &&
        price.from.day() == now.day() &&
        price.from.hour() == now.hour()
    }).ok_or(EmptyVec.into())
}

pub fn get_min_price(prices: &Vec<Price>) -> i32 {
    let mut sorted_prices = prices.clone();
    sorted_prices.sort_by(|a, b| a.value.cmp(&b.value));

    sorted_prices[0].value
}

pub fn get_max_price(prices: &Vec<Price>) -> i32 {
    let mut sorted_prices = prices.clone();
    sorted_prices.sort_by(|a, b| b.value.cmp(&a.value));

    sorted_prices[0].value
}

pub fn approve_lowest_prices(prices: Vec<Price>, count: usize) -> Vec<Price> {
    let mut approved_prices = prices.clone();
    approved_prices.sort_by(|a, b| a.value.cmp(&b.value));

    for i in 0..count {
        approved_prices[i].approved = true;
    }

    approved_prices.sort_by(|a, b| a.from.cmp(&b.from));
    approved_prices
}

async fn fetch_json(url: hyper::Uri) -> Result<Root> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(url).await?;
    let body = hyper::body::aggregate(res).await?;

    Ok(serde_json::from_reader(body.reader())?)
}
