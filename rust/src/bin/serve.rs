use clap::{Arg, Command};
use tide::Request;
use tide::prelude::*;
use prometheus::{
    HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Opts, Registry,  /*Counter, TextEncoder, Encoder*/
};
use lazy_static::lazy_static;

use testaroo::metrics::hello;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

// Application server Options
struct ServerOptions {
    port: u16
}

// Metric server options
// struct MetricsOptions {
//     port: u16
// }


async fn start_server(options: ServerOptions) -> tide::Result<()>{
    println!("Starting server");
    let mut server = tide::new();
    server.at("/orders/shoes").post(order_shoes);
    server.listen(format!("127.0.0.1:{}", options.port)).await?;
    Ok(())
}


lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    pub static ref INCOMING_REQUESTS: IntCounter =
        IntCounter::new("incoming_requests", "Incoming Requests").expect("metric can be created");

    pub static ref CONNECTED_CLIENTS: IntGauge =
        IntGauge::new("connected_clients", "Connected Clients").expect("metric can be created");

    pub static ref RESPONSE_CODE_COLLECTOR: IntCounterVec = IntCounterVec::new(
        Opts::new("response_code", "Response Codes"),
        &["env", "statuscode", "type"]
    )
    .expect("metric can be created");

    pub static ref RESPONSE_TIME_COLLECTOR: HistogramVec = HistogramVec::new(
        HistogramOpts::new("response_time", "Response Times"),
        &["env"]
    )
    .expect("metric can be created");
}

// fn metrics() {
//     let encoder = TextEncoder::new();

// }


#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("{}", hello());

    let matches = Command::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::new("file")
                 .short('f')
                 .long("file")
                 .takes_value(true)
                 .help("A cool file"))
        .arg(Arg::new("port")
                 .short('p')
                 .long("port")
                 .takes_value(true)
                 .required(true)
                 .validator(|s| s.parse::<usize>())
                 .help("The port to serve the main application on"))
        .get_matches();

    start_server(ServerOptions{
        port: matches.value_of_t("port").unwrap_or(8080)
    }).await?;
    
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}