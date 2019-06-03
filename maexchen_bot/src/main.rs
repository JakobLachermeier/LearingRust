extern crate serenity;

//use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    model::user::User,
    utils::MessageBuilder,
};

struct Handler;

impl EventHandler for Handler{
    fn message(&self, c: Context, msg: Message) {
        let user = msg.author;

        if msg.content.starts_with("!") {
            let mut message_content = msg.content;
            message_content = message_content.replace("!", "");
            let mut message_content = message_content.split(" ");
            let mut message_content: Vec<&str> = message_content.collect();
            let command = message_content[0];
            let parameters = message_content[1..].to_vec();
            let message = MessageBuilder::new().push("asdf");
            user.direct_message(|m| m.content("asdf"));
            handle_command(command, parameters);

        }


    }
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);

    }
}

fn handle_command(command: &str, parameters: Vec<&str>) {
    println!("Command: {}, parameters: {:?}", command, parameters);
    match command {

    }
}

fn main() {
    let token = String::from("");

    let mut client = Client::new(&token, Handler).expect("Err creating client.");

    if let Err(why) = client.start() {
        print!("Client error: {:?}", why);
    }


}
