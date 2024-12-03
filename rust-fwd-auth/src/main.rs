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

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hey there!")
}


#[derive(Embed)]
#[folder = "static/"]
struct Assets;


#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

