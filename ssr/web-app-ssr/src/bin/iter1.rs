///ITER1
use actix_files as fs;
use actix_web::{error,
                web,
                App,
                Error,
                HttpResponse,
                HttpServer,
                Result,
};
use tera::Tera;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("TERA >> Listening on: 127.0.0.1:8080");

    HttpServer::new(|| {
        // here we specify where templates are
        let tera = Tera::new(concat!
                             (env!("CARGO_MANIFEST_DIR"),
                              "/static/iter1/**/*",
                             )
        )
            .unwrap();

        App::new()
            // obsolete way of data init
            //.data(tera) // inject tera into app
            .app_data(
                web::Data::new(tera)
            )
            // static
            .service(fs::Files::new("/static",
                                    "./static",
            )
                     .show_files_listing()
            )
            // dynamic
            .service(
                web::resource("/")
                    .route(
                        web::get()
                            .to(index)
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


///
async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    ctx.insert("name", // template KEY
               "fOoOkUmE", // template VALUE
    );

    ctx.insert("role",
               "KiNg",
    );

    let s = tmpl
        .render("index.html",
                &ctx,
        )
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(s)
    )
}
