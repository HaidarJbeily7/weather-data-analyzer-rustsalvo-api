use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    data: HashMap<String, Value>,
}

impl Weather {
    pub fn new(paylaod: &HashMap<String, Value>) -> Self {
        Self {
            data: paylaod.clone(),
        }
    }
}
