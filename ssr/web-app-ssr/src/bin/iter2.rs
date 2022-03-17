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


///curl -X POST localhost:8080/tutors --data-raw "name=conan&role=barbarian"
async fn handle_post_tutor(tmpl: web::Data<tera::Tera>,
                           params: web::Form<Tutor>) -> Result<HttpResponse, Error> {
    
    let mut ctx = tera::Context::new();

    ctx.insert("name",
               &params.name,
    );

    ctx.insert("role",
               &params.role,
    );

    ctx.insert("god",
               "CROM",
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
            )
    );
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::{
        header::CONTENT_TYPE,
        header::HeaderValue,
        StatusCode,
    };

    //use actix_web::HttpResponseBuilder;
    
    use actix_web::dev::{
        //HttpResponseBuilder, 
        Service, // traiut
        ServiceResponse, // struct
    };
    use actix_web::test::{
        self,
        TestRequest,
    };

    
    #[actix_rt::test]
    //#[actix_web::test]
    async fn handle_post_1_unit_test() {
        let params =
            web::Form(
                Tutor {
                    name: "foookume".to_string(),
                    role: "king".to_string(),
                }
            );

        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/iter2/**/*",
        ))
            .unwrap();

        let webdata_tera = web::Data::new(tera);

        let resp = handle_post_tutor(webdata_tera,
                                     params,
        )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        // test FAIL
        //assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        assert_eq!(
            resp
                .headers()
                .get(CONTENT_TYPE)
                .unwrap(),

            HeaderValue::from_static("text/html")
        );
    }

    #[actix_rt::test]
    async fn handle_post_1_integration_test() {
        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/iter2/**/*",
        ))
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(
                    web::Data::new(tera)
                )
                .configure(app_config)
        )
            .await;

        let req = TestRequest::post()
            .uri("/tutors")
            .set_form(&Tutor {
                name: "ancistrus".to_string(),
                role: "ranunculus".to_string(),
            })
            .to_request();

        let resp: ServiceResponse =
            app
            .call(req)
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/html")
        );
    }
}
