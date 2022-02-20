use clap::{Arg, Command};


use tide::Request;
use tide::prelude::*;



#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}


struct ServerOptions {
    port: u16
}

async fn start_server(options: ServerOptions) -> tide::Result<()>{
    println!("Starting server");
    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen(format!("127.0.0.1:{}", options.port)).await?;
    Ok(())
}


#[async_std::main]
async fn main() -> tide::Result<()> {

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