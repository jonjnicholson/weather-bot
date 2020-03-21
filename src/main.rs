extern crate reqwest;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::vec::Vec;
use std::format;

const TOKEN : &str = "";

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!weather") {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            get_weather(ctx, msg).expect("this failed");
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&TOKEN, Handler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

fn get_weather(ctx: Context, msg: Message) -> Result<(), Box<dyn std::error::Error>> {

    let param = parse_input(&msg.content);
    let url = format!("http://wttr.in/{}?format=4", param);

    let body = reqwest::blocking::get(&url)?
    .text()?;

    if let Err(why) = msg.channel_id.say(&ctx.http, body) {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

fn parse_input(user_input: &str) -> String {
    let pieces: Vec<&str> = user_input.split(' ').collect();
    if let Some((_, multi_params)) = pieces.split_first() {
        if !multi_params.is_empty() {
            return multi_params.join(" ")
        } else {
            return String::from("")
        };
    };
    String::from("")
}
