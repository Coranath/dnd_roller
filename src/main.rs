use rand::random;
use std::fmt::write;
use std::num::ParseIntError;
use std::str::FromStr;
use std::io::{stdout, stdin, Write};
use std::fs::{read_to_string};
use std::time;
use pancurses::{initscr, endwin, Window};

fn pc_read_line(w: &Window) -> String {

    let mut input = w.getch().unwrap();

    let mut c = match input {

        pancurses::Input::Character(v) => { v },
        _ => '\0'

    };

    let mut str = String::from("");

    while c != '\n' {

        println!("{c}");

        str = format!("{}{}", str, c);

        input = w.getch().unwrap();

        c = match input {

            pancurses::Input::Character(v) => { v },
            _ => '\0'

        };
    }
    return str
}

fn sleep(time: u64) {
    std::thread::sleep(time::Duration::from_secs(time));
}

#[derive(Debug, Clone)]
struct Monster {
    
    ac:u8,
    attack:u8,
    damage:Vec<Dice>,
    hp:i32,
    name:String,
    perception:u8,    

}

impl Monster {
    fn new() -> Self {
        let mut d = Vec::new();
        Monster{ac:10,attack:0,damage:d,hp:50,name:String::from("Unknown"),perception:0}
    }
}

impl FromStr for Monster {

    type Err = ();

    fn from_str(input: &str) -> Result<Monster, Self::Err> {

        let mut mon = Monster::new();

        for attr in input.split(",") {

            let atttr: Vec<&str> = attr.split(":").collect();

            match atttr[0].trim() {
                "ac" => mon.ac = atttr[1].trim().parse::<u8>().unwrap(),
                "attack" => mon.attack = atttr[1].parse::<u8>().unwrap(),
                "damage" => mon.damage.push(Dice::from_str(atttr[1]).unwrap()),
                "hp" => mon.hp = atttr[1].parse::<i32>().unwrap(),
                "name" => mon.name = atttr[1].to_owned(),
                "perception" => mon.perception = atttr[1].parse::<u8>().unwrap(),
                _ => ()
                
            } 
            
        }

        return Ok(mon)

    }
}

#[derive(Debug)]
struct Player {
    
    hp:i32,
    perception:u8,
    attack:u8,
    damage:Vec<Dice>,
    ac:u8
    
}

#[derive(Debug, Clone)]
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
    fn roll(&self) -> u8 {
        let sides = match self {
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
            Dice::D20 => 20,
        };

        return (random::<u8>()%sides)+1
    }
}

fn roll_dice(w: &Window) {


    w.erase();
    w.printw("Please enter the size of the die you would like to roll: ");
    w.refresh();

    let str = pc_read_line(w);

    let d = match Dice::from_str(str.trim()) {
        Ok(val) => val,
        Err(_) => { 
            w.printw("You did not enter a valid choice. Please enter d4, d6, d8, d10, d12, or d20!");
            sleep(5);
            return
        }
    };

    w.printw(format!("You have chosen {d:?}, here is your roll: {}", d.roll()));
    w.getch();
}

fn add_monster(w: &Window, monsters: &mut Vec<Monster>, saved_monsters: &mut Vec<Monster>) {

    w.erase();

    w.printw("Please make a choice from this list or type n to add a new monster!");

    let mut counter = 1;

    for monster in saved_monsters.clone() {
        w.printw(format!("\n{counter}. {monster:?}"));
        counter += 1;
    }

    w.printw("\n");

    let input = pc_read_line(w);

    if input == "n" {
        w.printw("Please enter the name of the monster: ");
        let name = pc_read_line(w);
        w.printw("\nPlease enter the armor class of the monster: ");
        let ac = pc_read_line(w);
        w.printw("\nPlease enter the attack modifier of the monster: ");
        let attack = pc_read_line(w);
        w.printw("\nPlease enter the damage dice of the monster: ");
        let damage = pc_read_line(w);
        w.printw("\nPlease enter the hit points of the monster: ");
        let hp = pc_read_line(w);
        w.printw("\nPlease enter the perception of the monster: ");
        let perception = pc_read_line(w);

        monsters.push(Monster::from_str(&format!("name:{name} ac:{ac} hp:{hp} damage:{damage} attack:{attack} perception:{perception}")).unwrap());

    }

    let success = match input.parse::<u32>() {

        Ok(v) => { monsters.push(saved_monsters[usize::try_from(v).unwrap()].clone()); true},
        Err(_) => false

    };

    if !success {

        for monster in saved_monsters.clone() {
            if input == monster.name {
                monsters.push(monster);
            }
        }
    }

}

fn load_saved_monsters() -> Result<Vec<Monster>, &'static str> {

    let contents = read_to_string("./src/saved_monsters.sav").unwrap();

    let mut mons_list: Vec<Monster> = Vec::new();

    for mon in contents.split("}") {
        mons_list.push(Monster::from_str(mon).unwrap())   
    } 

    Ok(mons_list)

}

fn main() {

    let mut monsters: Vec<Monster> = Vec::new();

    let mut saved_monsters = load_saved_monsters().unwrap();

    eprintln!("{saved_monsters:?}");

    let window = initscr();

    loop {

        // window.resize(0,0);

        window.printw("Please make a choice from the list below:
        1. Roll a die
        2. Add a monster
        3. Add a player
        4. Have monsters attack
        0. Quit
        ");
        window.refresh();
        let c = window.getch().unwrap();

        match c {
            pancurses::Input::Character(v) => {
                match v {
                    '0' => break,
                    '1' => roll_dice(&window),
                    '2' => add_monster(&window, &mut monsters, &mut saved_monsters),
                    // "3" => add_player(),
                    // "4" => tick(),
                    _ => {window.printw(format!("{v} was not a valid input. Please try again!"));}
                }
            },
            _ => ()
        }

        window.erase();

    }    

    endwin();

    std::fs::write("./src/saved_monsters.sav", format!("{saved_monsters:?}"));
}