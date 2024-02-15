use dotenv::dotenv;
use salvo::prelude::*;

mod models;
use models::weather::Weather;

mod services;
use services::weather_api_service::{ThirdPartyServiceProvider, WeatherAPIService};

#[handler]
async fn health() -> &'static str {
    "Server Health!"
}

#[handler]
async fn analyze(req: &mut Request, _: &mut Depot, res: &mut Response) {
    let city = req.params().get("city").clone().unwrap();
    let api_service = WeatherAPIService { city: city.clone() };
    let data = api_service.get_weather().await.unwrap();
    let weather = Weather::new(&data);
    res.render(Json(weather));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenv().ok();

    let analyze_router = Router::with_path("/analyze/<city>").get(analyze);

    let router: Router = Router::with_path("").get(health).push(analyze_router);

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
