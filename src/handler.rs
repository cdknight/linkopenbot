extern crate serenity;
extern crate opener;
extern crate regex;

use serenity::{
    async_trait,
    model::{channel::{Message, Embed, EmbedFooter}, gateway::Ready},
    prelude::*,
};
use serde_json;
use crate::{ServerConfiguration, ApplicationConfiguration};

use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use std::cell::RefCell;

pub struct MessageHandler;

impl MessageHandler {
    fn process_message(&self, msg: &String, embeds: Option<&Vec<Embed>>, keywords: Option<&Vec<String>>, opened_links: Option<&Vec<String>>) {
        // Check if there are any links in the string

        // For "text" messages
        //
        let mut text_opened_links = vec![]; // We can store this

        if let Some(keywords) = keywords {
            let mut no_keywords = true;
            for keyword in keywords {
                if msg.contains(keyword) {
                    no_keywords = false;
                }
            }
            if (no_keywords && embeds.is_none()) {
                println!("No keywords");
                return;
            }
        }

        let url_regex = Regex::new(r"https?://(?:\w+\.?)+/\S*").unwrap();

        for link_w in url_regex.captures_iter(msg) {
            let mut link = &link_w[0];
            let link_len = link.len();
            let last_char = &link[link_len-1..link_len];

            if last_char == ")" { // end of embed, so strip parentheses, got lazy with regex
                link = &link[..link_len-1]
            }


            println!("link is {:?}", &link);
            if opened_links.is_some() {
                if !opened_links.unwrap().contains(&link.to_string()) {
                    opener::open(&link); // Possibly add a thing to check we're not opening the same link twice (make a list and check for duplicates)
                }
            }
            else {
                opener::open(&link); // Opened links is not defined, don't have to check; blindly open.
            }

            text_opened_links.push(String::from(link));
        }


        // for the embeds, just take all the embeds, loop through their fields, and run them through this method :D
        if let Some(embeds) = embeds {
            // println!("Embed: {:?}", embeds);
            for embed in embeds {
                // deal with
                // author, description, fields, footer, url, title

                let mut embed_long_string = String::new(); // Take the entire embed and literally just dump it into a string
                for field in &embed.fields {
                    // println!("The field is {:?}", field);

                    embed_long_string += &(String::from(" ") + &field.value);
                }

                if embed.description.is_some() { embed_long_string += &(String::from(" ") + &embed.description.as_ref().unwrap()); }
                if embed.footer.is_some() { embed_long_string += &(String::from(" ") + &embed.footer.as_ref().unwrap().text); }
                if embed.url.is_some() { embed_long_string += &(String::from(" ") + &embed.url.as_ref().unwrap()); }
                if embed.title.is_some() { embed_long_string += &(String::from(" ") + &embed.title.as_ref().unwrap()); }

                println!("embed_long_string is {}", embed_long_string);

                if text_opened_links.is_empty() {
                    self.process_message(&embed_long_string, None, keywords, None);
                }

                self.process_message(&embed_long_string, None, keywords, Some(&text_opened_links));
            }
        }
    }
}
#[async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        // This code is hugely inefficient

        let file = File::open("Config.json").expect("wanted an actual file");
        let reader = BufReader::new(file);

        let appconf: ApplicationConfiguration = serde_json::from_reader(reader).expect("Wanted to be able to parse Config.json but something bad happened.");
        let serverconf: Vec<ServerConfiguration> = appconf.server_configuration;

        if let Some(guild_id) = msg.guild_id { // use match or something
            for server in serverconf {
                if server.id  == guild_id.0 {
                    if let Some(keywordchannels) = server.keywordchannels {
                        if keywordchannels.contains(&msg.channel_id.0) {
                            println!("Message in keyword channel");
                            match server.keywords {
                                Some(keywords) => self.process_message(&msg.content, Some(&msg.embeds), Some(&keywords), None),
                                None => panic!("Please fix your Config.json file. For a server with keyword channels, you must have a separate keywords field.")
                            }
                        }
                    }

                    if let Some(productchannels) = server.productchannels {
                        if productchannels.contains(&msg.channel_id.0) {
                            println!("Message in product channel");
                            self.process_message(&msg.content, Some(&msg.embeds), None, None)
                        }
                    }

                }
            }
        }
    }
}
