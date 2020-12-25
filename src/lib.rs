use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ServerConfiguration {
    pub id: u64,
   
    pub keywordchannels: Option<Vec<u64>>,
    pub keywords: Option<Vec<String>>,

    pub productchannels: Option<Vec<u64>>,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationConfiguration {
    pub token: String,
    pub server_configuration: Vec<ServerConfiguration>
}

pub mod handler;
