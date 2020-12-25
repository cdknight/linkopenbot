extern crate serenity;
extern crate serde_json;


use linkopenbot::handler::MessageHandler;
use linkopenbot::ApplicationConfiguration;
use serenity::{
    async_trait,
    model::{channel::{Message, Embed, EmbedFooter}, gateway::Ready},
    prelude::*,
};
use std::io::BufReader;
use std::fs::File;

#[tokio::main]
async fn main() {
    let file = File::open("Config.json").expect("wanted an actual file");
    let reader = BufReader::new(file);
    let appconf: ApplicationConfiguration = serde_json::from_reader(reader).expect("Wanted to be able to parse Config.json but something bad happened.");

    let token = appconf.token;
    let mut client = Client::builder(token)
        .event_handler(MessageHandler)
        .await.expect("wanted a token that worked.");



    if let Err(e) = client.start().await {
        println!("An error happened when connecting to Discord {:?}", e);
    }


}

