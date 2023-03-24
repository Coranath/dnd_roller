use rand::prelude::*;
use std::str::FromStr;
use std::io::{stdout, stdin, Write};

#[derive(Debug)]
enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20
}

impl FromStr for Dice {

    type Err = ();

    fn from_str(input: &str) -> Result<Dice, Self::Err> {
        match input {
            "d4" => Ok(Dice::D4),
            "d6" => Ok(Dice::D6),
            "d8" => Ok(Dice::D8),
            "d10" => Ok(Dice::D10),
            "d12" => Ok(Dice::D12),
            "d20" => Ok(Dice::D20),
            _ => Err(())
        }
    }
}

impl Dice {
    fn roll(&self) {
        let sides = match self {
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
            Dice::D20 => 20,
        };

        (rand::random::<u8>()%sides)+1
    }
}


fn main() {

    loop {

        print!("Please enter the size of the die you would like to roll: ");
        stdout().flush().unwrap();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let d = match Dice::from_str(&mut input.trim()) {
            Ok(val) => val,
            Err(_) => { 
                println!("You did not enter a valid choice. Please enter d4, d6, d8, d10, d12, or d20!");
                continue
            }
        };

        println!("{d:?}");

        

        println!("You have chosen {d:?}, here is your roll: {}", d::roll());
    
    }
}