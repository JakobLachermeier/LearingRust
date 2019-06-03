extern crate serenity;

//use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    model::user::User,
    utils::MessageBuilder,
};
use std::time::Instant;

struct Handler;

enum Symbol {
    Rock,
    Paper,
    Scissors,
}

struct KliKlaKluGame {
    player_1: User,
    player_2: User,
    player_1_symbol: Symbol,
    player_2_symbol: Symbol,
    creation_time: Instant,
}

struct KliKlaKluCurrentGames {
    open_games: Vec<KliKlaKluGame>,
}

impl KliKlaKluGame {
    fn new(initiator: User) -> KliKlaKluGame {
        let game = KliKlaKluGame{
            player_1: None,
            player_2: None,
            player_1_symbol: None,
            player_2_symbol: None,
            creation_time: Instant::now(),
        };
    }
}

impl KliKlaKluCurrentGames {
    fn addGame(&mut self, initiator: User) {
        let mut game = KliKlaKluGame::new(initiator);
        self.open_games.push(game);
    }
}

impl EventHandler for Handler{
    fn message(&self, c: Context, msg: Message) {
        let author = msg.author;

        if msg.content.starts_with("!") {
            let mut message_content = msg.content;
            message_content = message_content.replace("!", "");
            let mut message_content = message_content.split(" ");
            let mut message_content: Vec<&str> = message_content.collect();
            let command = message_content[0];
            let parameters = message_content[1..].to_vec();
            let message = MessageBuilder::new().push("asdf");
            user.direct_message(|m| m.content("asdf"));
            handle_command(command, parameters, author);
        }


    }
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);

    }
}

fn handle_command(command: &str, parameters: Vec<&str>, author: User) {
    match command {
        "kliklaklu" => kliklaklu(),
        _ => println!("Wrong command."),
    }
}

fn kliklaklu() {

}

fn main() {
    let token = String::from();

    let mut client = Client::new(&token, Handler).expect("Err creating client.");

    if let Err(why) = client.start() {
        print!("Client error: {:?}", why);
    }
}


/**
The user types !kliklaklu in the chat.
He receives a private message telling him he has to wait for another player to join.
The bot sends out a message in the global chat prompting a user to join the initiator
by typing !accept in the chat.

The bot asks both players to send their choice
it posts the result in both the private chats and the public one
if its a draw both players have to type their choice again.

*/