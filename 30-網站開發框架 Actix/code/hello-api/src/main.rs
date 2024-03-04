use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is on http://127.0.0.1:9527");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(echo)
            .route("/hey", web::get().to(greeting))
    })
        .bind(("127.0.0.1", 9527))?
        .run()
        .await
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn greeting() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}