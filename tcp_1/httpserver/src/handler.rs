///HANDLER
use http::{
    httprequest::HttpRequest,
    httpresponse::HttpResponse,
};
use serde::{Deserialize,
            Serialize,
};
use std::collections::HashMap;
use std::env;
use std::fs;


pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;

    /// httpserver root folder
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public",
                                   env!("CARGO_MANIFEST_DIR"),
        );

        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

        // better with Path join
        let full_path = format!("{}/{}",
                                public_path,
                                file_name,
        );

        // Result<String>
        let contents = fs::read_to_string(full_path);

        // Result -> Option
        contents.ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        // our own Struct -> status_code / headers / body
        HttpResponse::new("404",
                          None,
                          Self::load_file("404.html"),
        )
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        // Get the path of static page resource being requested
        let http::httprequest::Resource::Path(s) = &req.resource;

        // Parse the URI
        let route: Vec<&str> = s.split("/").collect();

        match route[1] {
            "" => HttpResponse::new("200",
                                    None,
                                    Self::load_file("index.html",
                                    )
            ),

            "health" => HttpResponse::new("200",
                                          None,
                                          Self::load_file("health.html"),
            ),

            // any other file for static
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();

                    // hardcoded as not care about txt/png/csv/...
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }

                    HttpResponse::new("200",
                                      Some(map),
                                      Some(contents))
                },
                
                None => HttpResponse::new("404",
                                          None,
                                          Self::load_file("404.html"),
                ),
            },
        }
    }
}

// Define a load_json() method to load orders.json file from disk
impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data",
                                   env!("CARGO_MANIFEST_DIR"),
        );
        
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);

        let full_path = format!("{}/{}",
                                data_path,
                                "orders.json",
        );

        let json_contents = fs::read_to_string(full_path);

        let orders: Vec<OrderStatus> = serde_json::from_str(
            json_contents
                .unwrap()
                .as_str()
        )
            .unwrap();

        orders
    }
}

// Implement the Handler trait
impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;

        // Parse the URI
        let route: Vec<&str> = s.split("/").collect();

        // just to do not panic for my debug
        let len = route.len();

        //println!("ROUTE: {route:?} / {len}");

        if len <= 2 {
            //println!("<=2");
            return HttpResponse::new("404",
                                     None,
                                     Self::load_file("404.html"),
            )
        }

        // if route if /api/shipping/orders, return json
        //
        // ROUTE: ["", "api", "shipping"]
        // as hardcoded index this will fail at:
        // localhost:3000/api
        // localhost:3000/api/shipping
        match route[2] {
            //"shipping" if route.len() > 2 && route[3] == "orders" => {
            "shipping" if route.len() > 3 && route[3] == "orders" => {
                let body =
                    Some(
                        serde_json::to_string(
                            &Self::load_json() // stil load orders.json
                        )
                            .unwrap()
                    );
                
                let mut headers: HashMap<&str, &str> = HashMap::new();

                headers.insert("Content-Type", "application/json");

                HttpResponse::new("200",
                                  Some(headers),
                                  body,
                )
            },
            
            _ => HttpResponse::new("404",
                                   None,
                                   Self::load_file("404.html"),
            ),
        }
    }
}
