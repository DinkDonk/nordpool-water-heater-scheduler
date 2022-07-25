use chrono::prelude::*;
use tokio_cron_scheduler::{Job, JobScheduler};
use config::Config;

mod nordpool;
mod display;
mod wall_socket;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn run_job() -> Result<()> {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("Settings"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let enable_display: bool = settings.get::<bool>("display").expect("`display` needs to be set in Settings.toml");
    let prices = nordpool::get_prices(Some(Local::today())).await?;

    let prices = nordpool::approve_lowest_prices(prices, 8);
    let is_price_approved = nordpool::get_current_price(&prices)?.approved;

    wall_socket::toggle_switch(is_price_approved)?;

    if enable_display {
        display::draw(prices);
    }

    Ok(())
}

#[tokio::main]
async fn main() {

    // Run job once on start-up
    match run_job().await {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    }

    // Run job every hour
    let sched = JobScheduler::new().unwrap();

    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();

    let get_prices_job = sched.add(Job::new_async("0 0 * * * *", |_uuid, _l| {
        Box::pin(async move {
            match run_job().await {
                Ok(_) => (),
                Err(e) => println!("{}", e)
            }
        })
    }).unwrap());

    if get_prices_job.is_err() {
        println!("Error starting get data job");
        return;
    }

    let start = sched.start();

    if start.is_err() {
        println!("Error starting scheduler");
        return;
    }

    loop {}
}
