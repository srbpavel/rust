use actix_files as fs;
use actix_web::{error,
                web,
                App,
                Error,
                HttpResponse,
                HttpServer,
                Result,
};
use serde::{Deserialize,
            Serialize,
};
use tera::Tera;

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {

    let tutors: Vec<Tutor> = vec![
        Tutor {
            name: String::from("conan"),
        },
        Tutor {
            name: String::from("subotai"),
        },
        Tutor {
            name: String::from("sonya"),
        },
        Tutor {
            name: String::from("crom"),
        },
    ];

    let mut ctx = tera::Context::new();

    ctx.insert("tutors",
               &tutors,
    );

    ctx.insert("text",
               &"List of all tutors!".to_owned(),
    );

    let s = tmpl
        .render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(s)
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("TERA [3] >>> Listening on: 127.0.0.1:8080!");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/iter3/**/*",
        ))
            .unwrap();

        App::new()
            //.data(tera)
            .app_data(
                web::Data::new(tera)
            )
            .service(
                fs::Files::new(
                    "/static",
                    "./static",
                )
                    .show_files_listing()
            )
            .service(
                web::resource("/tutors")
                    .route(
                        web::get()
                            .to(handle_get_tutors)
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
