use rand::Rng;
use std::io;

#[derive(Debug)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Winner {
    Computer,
    Human,
    Draw,
}

impl Winner {
    fn get_winner_message(&self) -> &str {
        match *self {
            Winner::Computer => "The computer won",
            Winner::Human => "You won.",
            Winner::Draw => "It's a draw.",
        }
    }
}


fn get_user_symbol() -> Symbol {
    loop {
        println!("Enter r, p, or s.");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line.");
        user_input = user_input.replace("\n", "");
        user_input = user_input.replace("\r", "");
        user_input = user_input.to_lowercase();
        match user_input.as_ref() {
            "r" => return Symbol::Rock,
            "p" => return Symbol::Paper,
            "s" => return Symbol::Scissors,
            _ => continue
        }
    }
}

fn get_random_symbol() -> Symbol {
    match rand::thread_rng().gen_range(0, 3) {
        0 => return Symbol::Rock,
        1 => return Symbol::Paper,
        _ => return Symbol::Scissors,
    }
}

fn get_winner(player_symbol: Symbol, computer_symbol: Symbol) -> Winner {
    match player_symbol {
        Symbol::Rock => {
            match computer_symbol {
                Symbol::Paper => Winner::Computer,
                Symbol::Scissors => Winner::Human,
                Symbol::Rock => Winner::Draw,
            }
        }
        Symbol::Paper => {
            match computer_symbol {
                Symbol::Rock => Winner::Human,
                Symbol::Scissors => Winner::Computer,
                Symbol::Paper  => Winner::Draw,
            }
        }
        Symbol::Scissors => {
            match computer_symbol {
                Symbol::Rock => Winner::Computer,
                Symbol::Paper =>Winner::Human,
                Symbol::Scissors => Winner::Draw,
            }
        }
    }
}

fn main() {
    let user_symbol = get_user_symbol();
    let computer_symbol = get_random_symbol();
    println!("You played {:?}, the computer played {:?}.", user_symbol, computer_symbol);
    let winner = get_winner(user_symbol, computer_symbol);
    println!("{}", winner.get_winner_message());
}
