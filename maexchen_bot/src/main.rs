extern crate serenity;

//use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    model::user::User,
};

struct Handler;

impl EventHandler for Handler{
    fn message(&self, c: Context, msg: Message) {
        let user = msg.author;
        c.shard.websocket_message()

        if msg.content.starts_with("!") {
            let mut message_content = msg.content.replace("!", "");
            let mut message_content = message_content.split(" ");
            let mut message_content: Vec<&str> = message_content.collect();
            let command = message_content[0];
            let parameters = message_content[1..].to_vec();
            handle_command(command, parameters);
        }


    }
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);

    }
}

fn handle_command(command: &str, parameters: Vec<&str>) {
    println!("Command: {}, parameters: {:?}", command, parameters);
}

fn main() {
    let token = String::from("NTgxNTI3NzU2MTIzMjc1Mjg2.XOglfA.zore4YXAcXwd0HegCqGhaawWOgo");

    let mut client = Client::new(&token, Handler).expect("Err creating client.");

    if let Err(why) = client.start() {
        print!("Client errer: {:?}", why);
    }
    let u = User::;

}
