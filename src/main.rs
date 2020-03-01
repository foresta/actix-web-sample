use actix_web::{web, App, HttpServer, HttpResponse, Error, error};
use tera::Tera;

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut ctx = tera::Context::new();
    ctx.insert("name", "kz_morita");
    let view =
        tmpl.render("index.html.tera", &ctx)
            .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        let templates = Tera::new("templates/**/*").unwrap();

        App::new()
            .data(templates)
            .service(web::resource("/").to(index))
    })
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run()
        .await
}
