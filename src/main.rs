use chrono::prelude::*;
use tokio_cron_scheduler::{Job, JobScheduler};

mod nordpool;
mod display;
mod wall_socket;

async fn run_job() {
    let prices = nordpool::get_prices(Some(Local::today())).await;
    let prices = nordpool::approve_lowest_prices(prices.unwrap(), 8);

    let price = nordpool::get_current_price(&prices);

    match price {
        Some(price) => {
            match wall_socket::toggle_switch(price.approved) {
                Err(e) => println!("{:?}", e),
                _ => ()
            }
        },
        None => println!("Could not find {} in prices", Local::today())
    }

    // display::print_graph(prices);
}

#[tokio::main]
async fn main() {
    // Run job once on start-up
    run_job().await;

    // Run job every hour
    let sched = JobScheduler::new().unwrap();

    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();

    let get_prices_job = sched.add(Job::new_async("0 0 * * * *", |_uuid, _l| {
        Box::pin(async move {
            run_job().await;
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
