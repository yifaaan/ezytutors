use actix_files as fs;
use actix_web::{
    error,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer, Result,
};
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

async fn handle_get_tutors(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let tutors: Vec<Tutor> = vec![
        Tutor {
            name: "tutor 1".to_string(),
        },
        Tutor {
            name: "tutor 2".to_string(),
        },
    ];
    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutors);
    // 向html文件插入数据
    let rendered_html = tmpl
        .render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .json(rendered_html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8080");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter3/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/tutors").route(web::get().to(handle_get_tutors)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
