use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn get_dice_rolls() -> Vec<u8> {
    //! Reads the command line and parses 2 numbers separated by whitespace.
    let mut numbers: Vec<u8>;
    loop {
        println!("Please enter your dice rolls separated by whitespace.");
        numbers = vec![];
        let mut dice_roll = String::new();

        io::stdin().read_line(&mut dice_roll).expect("Failed to read line.");
        dice_roll = dice_roll.replace("\n", "");
        dice_roll = dice_roll.replace("\r", "");
        let dice_roll = dice_roll.split(" ");
        let vec: Vec<&str> = dice_roll.collect();

        if vec.len() != 2 {
            continue;
        }
        for &value in vec.iter() {
            let number: u8 = match String::from(value).parse() {
                Ok(return_number) => return_number,
                Err(_) => continue
            };
            if number > 6 || number <= 0 {
                continue;
            }
            numbers.push(number);
        }
        if numbers.len() == 2 {
            break;
        }
    }
    numbers.sort();
    return numbers;
}


fn calculate_points(numbers: Vec<u8>) -> u16 {
    //! Takes in a sorted vector with length 2 and returns points.
    let mut points: u16 = 0;
    if numbers[0] == numbers[1] {
        return numbers[0] as u16 * 100;
    } else if numbers[0] == 1 && numbers[1] == 2 {
        points += 1000;
    } else {
        points += (numbers[1] as u16 * 10 + numbers[0] as u16) * 10;
    }
    return points;
}

fn get_random_points() -> u16 {
    let mut random_numbers: Vec<u8> = (0..2).map(|_| {
        rand::thread_rng().gen_range(1, 7)
    }).collect();
    random_numbers.sort();
    return calculate_points(random_numbers);
}

fn main() {
    loop {
        let numbers = get_dice_rolls();
        let player_points = calculate_points(numbers);
        let computer_points = get_random_points();
        println!("Your points: {}. Computer points: {}.", player_points, computer_points);
        match player_points.cmp(&computer_points) {
            Ordering::Less => println!("The computer wins."),
            Ordering::Greater => println!("You win."),
            Ordering::Equal => println!("It's a draw.")
        }
    }
}
