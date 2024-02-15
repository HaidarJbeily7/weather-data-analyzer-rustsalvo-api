use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait ThirdPartyServiceProvider {
    async fn get_weather(&self) -> Result<HashMap<String, Value>, String>;
}

pub struct WeatherAPIService {
    pub city: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherAPIServiceProviderResponse {
    location: HashMap<String, Value>,
    current: HashMap<String, Value>,
}

impl ThirdPartyServiceProvider for WeatherAPIService {
    async fn get_weather(&self) -> Result<HashMap<String, Value>, String> {
        let weather_api_token =
            std::env::var("WEATHER_API_KEY").expect("Please Set the environment variable");

        let query = &[
            ("key".to_owned(), weather_api_token),
            ("q".to_owned(), self.city.clone()),
            ("aqi".to_owned(), "no".to_owned()),
        ];
        let client = reqwest::Client::new();
        let response = client
            .get("http://api.weatherapi.com/v1/current.json")
            .query(query)
            .send()
            .await
            .unwrap();
        let json: WeatherAPIServiceProviderResponse = response.json().await.unwrap();
        let mut payload = HashMap::new();
        for (key, value) in json.location.iter() {
            payload.insert(format!("location_{}", key.to_owned()), value.to_owned());
        }
        for (key, value) in json.current.iter() {
            payload.insert(format!("current_{}", key.to_owned()), value.to_owned());
        }

        Ok(payload)
    }
}
