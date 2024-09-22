use std::env;

use actix_files as fs;
// X use actix_web::client::Client;
use actix_web::web::Data;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use awc::Client;
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let client = Client::new(); // Client::default();

    // Create request builder and send request
    // get data from db
    let response = client
        .get("http://localhost:3000/tutors/")
        .send() // <- Send request
        .await
        .unwrap()
        .body()
        .await
        .unwrap();

    let str_list = std::str::from_utf8(&response.as_ref()).unwrap();
    let tutor_list: Vec<Tutor> = serde_json::from_str(str_list).unwrap();
    let mut ctx = tera::Context::new();
    // 插入获得的数据
    ctx.insert("tutors", &tutor_list);
    let s = tmpl
        .render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    // println!("Listening on: 127.0.0.1:8080");
    println!("Listening on: {}", addr);
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter4/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/tutors").route(web::get().to(handle_get_tutors)))
    })
    .bind(addr)?
    .run()
    .await
}
