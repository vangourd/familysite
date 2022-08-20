use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs; 
use askama::Template;

// TEMPLATING

#[derive(Template)]
#[template(path="index.html")]
struct HomeTemplate<'a> {
    site_title: &'a str
}

#[derive(Template)]
#[template(path="login.html")]
struct LoginTemplate<'a>{
    site_title: &'a str
}

// ROUTES
#[get("/")]
async fn home() -> impl Responder {
    let home = HomeTemplate { 
            site_title: "Welcome to the LoganFamily Site",
        };
    HttpResponse::Ok().body(home.render().unwrap())
}

#[get("/login")]
async fn login() -> impl Responder {
    let page = LoginTemplate {
        site_title: "Login",
    };
    HttpResponse::Ok().body(page.render().unwrap())
}

#[post("/login")]
async fn login_submission() -> impl Responder {
    let page = LoginTemplate {
        site_title: "Login"
    };
    HttpResponse::Ok().body(page.render().unwrap())
}

// MAIN LOOP

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(login)
            .service(fs::Files::new("/static","static/").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}