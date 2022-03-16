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
    role: String,
}

// store tera template in application state
async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("form.html",
                &tera::Context::new(),
        )
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(s)
    )
}


// curl -X POST localhost:8080/tutors -d "name=Terry"
async fn handle_post_tutor(tmpl: web::Data<tera::Tera>,
                           params: web::Form<Tutor>) -> Result<HttpResponse, Error> {
    
    let mut ctx = tera::Context::new();

    ctx.insert("name",
               &params.name,
    );

    ctx.insert("role",
               &params.role,
    );

    ctx.insert("text",
               "tea time: dj thurbo || raw puerh",
    );

    let s = tmpl
        .render("user.html",
                &ctx,
        )
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(s)
    )
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("TERA [2] >>> Listening on: 127.0.0.1:8080");

    HttpServer::new(|| {
        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/iter2/**/*",)
        )
            .unwrap();

        App::new()
            //.data(tera)
            .app_data(
                web::Data::new(tera)
            )
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(
                web::resource("/")
                    .route(
                        web::get()
                            .to(index)
                    )
            )
            .service(
                web::resource("/tutors")
                    .route(
                        web::post()
                            .to(handle_post_tutor)
                    )
            )//,
    );
}
