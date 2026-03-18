use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    pub resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec::<ProviderConfig>,
}