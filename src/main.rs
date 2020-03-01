use actix_web::{dev, web, http, App, HttpServer, HttpResponse, Error, error};
use actix_web::middleware::{errhandlers::ErrorHandlers, errhandlers::ErrorHandlerResponse};
use actix_files as fs;
use tera::Tera;

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut ctx = tera::Context::new();
    ctx.insert("name", "kz_morita");
    let view =
        tmpl.render("index.html.tera", &ctx)
            .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

fn not_found<B>(res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let new_resp = fs::NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {

        let error_handlers = ErrorHandlers::new()
            .handler(http::StatusCode::NOT_FOUND, not_found);
        let templates = Tera::new("templates/**/*").unwrap();

        App::new()
            .data(templates)
            .wrap(error_handlers)
            .service(web::resource("/").to(index))
    })
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run()
        .await
}
