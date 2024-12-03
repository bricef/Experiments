use ntex::web;
use rust_embed::Embed;

#[web::get("/")]
async fn hello() -> impl web::Responder {
    let index = Assets::get("index.html").unwrap();
    let bs = index.data.into_owned();
    let d = std::str::from_utf8(&bs).unwrap().to_owned();
    web::HttpResponse::Ok()
        .content_type("text/html")
        .body(d)
}

#[derive(Embed)]
#[folder = "static/"]
struct Assets;

#[web::get("/static/{filename:.*}")]
async fn static_files(path: web::types::Path<String>) -> impl web::Responder {
    let filepath = path.into_inner();
    let file = Assets::get(&filepath).unwrap();
    let bs = file.data.into_owned();
    let d = std::str::from_utf8(&bs).unwrap().to_owned();
    web::HttpResponse::Ok().body(d)
}



// use serde::Deserialize;

// #[derive(Deserialize)]
// struct Info {
//     username: String,
// }

// /// deserialize `Info` from request's body
// #[web::post("/submit")]
// async fn submit(info: web::types::Json<Info>) -> Result<String, web::Error> {
//     Ok(format!("Welcome {}!", info.username))
// }


// GET / -> /login
// GET /login 
// POST /login
// GET /logout
// POST /authz/fwdAuth
// GET /health
// GET /metrics
// GET /ready
// GET /info


// Principal | Resource | Action 




#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(hello)
            .service(static_files)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

