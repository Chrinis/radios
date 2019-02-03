use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawList {
    pub radio_stations: HashMap<String, String>,
}

pub struct Station {
    pub name: String,
    pub url: String,
}

impl Station {
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self::_new(name.into(), url.into())
    }

    fn _new(name: String, url: String) -> Self {
        Self {
            name,
            url,
        }
    }
}
