use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .route("/", web::get().to(login))
            .route("/home", web::get().to(home))
            .route("/home", web::post().to(home))
            .route("/table-basic", web::get().to(table_basic))
            .route("/table-full", web::get().to(table_full))
            .service(Files::new("/css", "./static/css"))
            .service(Files::new("/js", "./static/js"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn login(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({});
    let body = hb.render("login", &data).unwrap();
    HttpResponse::Ok().body(body)
}

async fn home(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({});
    let body = hb.render("home", &data).unwrap();
    HttpResponse::Ok().body(body)
}

async fn table_basic(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({});
    let body = hb.render("table-basic", &data).unwrap();
    HttpResponse::Ok().body(body)
}

async fn table_full(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({});
    let body = hb.render("table-data-table", &data).unwrap();
    HttpResponse::Ok().body(body)
}
