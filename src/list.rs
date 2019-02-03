use crate::{
    model::Station,
};
use std::collections::HashMap;
use toml::de::Error as TomlDeError;

pub struct RadioList {
    pub messages: Vec<String>,
    pub stations: HashMap<String, Station>,
}

impl RadioList {
    pub fn new() -> Result<Self, TomlDeError> {
        let stations = crate::all()?.iter().map(|(name, url)| (
            name.to_lowercase(),
            Station::new(name.to_owned(), url.to_owned()),
        )).collect::<HashMap<String, Station>>();

        Ok(Self {
            messages: vec![],
            stations,
        })
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&Station> {
        self._get(name.as_ref())
    }

    fn _get(&self, name: &str) -> Option<&Station> {
        self.stations.get(&name.to_lowercase())
    }
}
