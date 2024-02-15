use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    data: HashMap<String, String>,
}

impl Weather {
    pub fn new(paylaod: &HashMap<String, String>) -> Self {
        Self {
            data: paylaod.clone(),
        }
    }
}
