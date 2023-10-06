use actix_files::Files;
use actix_web::cookie::Cookie;
use actix_web::{http, web, App, HttpResponse, HttpServer};
use askama::Template;
use serde::Deserialize;

#[derive(Deserialize, Template)]
#[template(path = "login.html")]
struct LoginForm {
    username: String,
    password: String,
    error_message: Option<String>,
}

impl LoginForm {
    pub fn new() -> Self {
        LoginForm {
            username: "".to_string(),
            password: "".to_string(),
            error_message: None,
        }
    }
}

#[derive(Template)]
#[template(path = "secured/template/base.html")]
struct BaseTemplate;

#[derive(Template)]
#[template(path = "secured/views/dashboard.html")]
struct DashboardTemplate;

#[derive(Template)]
#[template(path = "secured/views/users/search.html")]
struct UsersTemplate;

#[derive(Template)]
#[template(path = "secured/views/users/new-user.html")]
struct NewUserTemplate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/")
                    .route(web::get().to(login))
                    .route(web::post().to(authenticate)),
            )
            .route("/dashboard", web::get().to(dashboard))
            .route("/logout", web::get().to(logout))
            // users routes
            .service(web::resource("/users")
                .route(web::get().to(users)))
            .service(web::resource("/users/new-user")
                .route(web::get().to(user_new))
            )
            // static resource
            .service(Files::new("/css", "./static/css"))
            .service(Files::new("/js", "./static/js"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn login() -> HttpResponse {
    println!(" showing login page ....");
    let s = LoginForm::new().render().unwrap();
    HttpResponse::Ok().body(s)
}

async fn logout() -> HttpResponse {
    println!(" logout  ....");
    HttpResponse::SeeOther()
        .insert_header((http::header::LOCATION, "/"))
        .finish()
}

async fn authenticate(form: web::Form<LoginForm>) -> HttpResponse {
    println!("processing authentication .... username: {}", form.username);
    if form.username == "ram" {
        HttpResponse::SeeOther()
            .cookie(
                Cookie::build("my_auth_cookie", "SomeValue")
                    .http_only(true) // for security
                    .finish(),
            )
            .insert_header((http::header::LOCATION, "/dashboard"))
            .finish()
    } else {
        let s = LoginForm {
            username: form.username.clone(),
            password: "".to_string(),
            error_message: Some("Invalid credentials".to_string()),
        }
        .render()
        .unwrap();
        HttpResponse::Ok().body(s)
    }
}

async fn dashboard() -> HttpResponse {
    let s = DashboardTemplate.render().unwrap();
    HttpResponse::Ok().body(s)
}

async fn users() -> HttpResponse {
    let s = UsersTemplate.render().unwrap();
    HttpResponse::Ok().body(s)
}

async fn user_new() -> HttpResponse {
    let s = NewUserTemplate.render().unwrap();
    HttpResponse::Ok().body(s)
}