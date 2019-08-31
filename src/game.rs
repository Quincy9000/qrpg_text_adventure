use crate::player::*;
use console::Term;
use std::fs::*;
use std::io::prelude::*;

pub fn input() -> String {
    let term = Term::stdout();
    term.read_line().unwrap().trim().to_owned()
}

pub fn key_to_continue() {
    println!("Press any key to continue");
    console::Term::stdout().read_key().unwrap();
}

pub fn load_game() {
    println!("Well who are you going to play as?");

    println!("Input the character you want to play(You did make a character, riight?)..");

    let name = input();

    if !std::path::Path::new(&format!("saves/{}.txt", name)).exists() {
        println!("YOU CANNOT PLAY WITHOUT A CHAR YOU NOOB!");
        key_to_continue();
        return;
    }

    println!("So this game is a randomized rpg adventure game! You will die and your character will be deleted on death!\nHowever don't be afraid as you can always make a new one and play again and get a totally different random start!");

    key_to_continue();
}

pub fn character_creator() {
    let term = Term::stdout();

    println!("What is your name?");

    let name: String = loop {
        let n = input();
        let format = format!("{}.txt", n);

        if std::path::Path::new(&format).exists() {
            println!("That name already exists! Please choose another.");
            continue;
        }

        break n;
    };

    let format = format!("saves/{}.txt", name);

    let mut selection = 1;
    let max = 3;
    let mut gender = Gender::Other;

    //term.clear_screen().unwrap();

    loop {
        term.write_line("What is your Gender?").unwrap();

        match selection {
            1 => term.write_line("1: Male <--\n2: Female\n3: Other").unwrap(),
            2 => term.write_line("1: Male\n2: Female <--\n3: Other").unwrap(),
            3 => term.write_line("1: Male\n2: Female\n3: Other <--").unwrap(),
            _ => term.write_line("1: Male\n2: Female\n3: Other").unwrap(),
        }

        let k = term.read_key().unwrap();

        if k == console::Key::ArrowUp {
            selection -= 1;
            if selection < 1 {
                selection = 1;
            }
        }

        if k == console::Key::ArrowDown {
            selection += 1;
            if selection > max {
                selection = max;
            }
        }

        if k == console::Key::Enter {
            if selection == 1 {
                gender = Gender::Male;
            } else if selection == 2 {
                gender = Gender::Female;
            } else if selection == 3 {
                gender = Gender::Other;
            }
            break;
        }

        //term.clear_screen().unwrap();
    }

    //term.clear_screen().unwrap();

    println!("How old are you, {}", name);

    let age: i32 = loop {
        match input().parse() {
            Ok(n) => break n,
            Err(_) => println!("Bad Input, try again."),
        };
    };

    let mut p1 = Player::new(&name, gender, age);

    let mut points_left = 5;

    println!("Time to allocate points! Where do you want them to go?");

    while points_left > 0 {
        if points_left == 5 {
            println!("Where do you want the first point?");
        } else if points_left > 1 {
            println!("Where do you want the next point?");
        } else {
            println!("Where do you want the last point?");
        }

        println!("P, T, or M?");

        let input = term.read_char().unwrap().to_ascii_uppercase();

        let point = if input == 'P' || input == 'T' || input == 'M' {
            points_left -= 1;
            input
        } else {
            println!("Bad Input! Try again.");
            continue;
        };

        if point == 'P' {
            p1.stats.physique += 1;
        } else if point == 'T' {
            p1.stats.technique += 1;
        } else {
            p1.stats.mystique += 1;
        }
    }

    println!("Ok! Here is your character Page!\n{}", p1);

    println!("Creating save data for {}.", p1.name);

    let mut file = File::create(format).expect("Failed to create player file.");

    let s = format!(
        "Name: {}\nAge: {}\nGender: {}\nStats: {}",
        p1.name, p1.age, p1.gender, p1.stats,
    );

    println!("{}", s);

    file.write_all(s.as_bytes()).expect("Fucking Error!");

    println!("Player file created!");

    term.read_key().unwrap();
}
