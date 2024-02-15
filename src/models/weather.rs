use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    city: String,
}
impl Weather {
    pub fn new(city: &str) -> Self {
        Self {
            city: String::from(city)
        }
    }
}

pub trait WeatherTrait {
    fn get_weather(&self) -> String;
}

impl WeatherTrait for Weather {
    fn get_weather(&self) -> String {
        format!("Hello, {}!", self.city)
    }
}

