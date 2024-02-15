use std::collections::HashMap;

use salvo::prelude::*;

mod models;
use models::weather::{Weather, WeatherTrait};

#[handler]
async fn health() -> &'static str {
    "Server Health!"
}

#[handler]
async fn analyze(req: &mut Request, _: &mut Depot, res: &mut Response) {
    let city = req.params().get("city").clone().unwrap();
    let weather= Weather::new(city);
    print!("Health, {}!", weather.get_weather());
    let mut response_body = HashMap::new();
    response_body.insert("city", weather.get_weather());
    res.render(Json(response_body));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();


    let analyze_router = Router::with_path("/analyze/<city>").get(analyze);

    let router: Router = Router::with_path("")
                                .get(health)
                                .push(analyze_router);
    
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}