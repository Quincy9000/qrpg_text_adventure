use crate::player::*;
use console::Term;
use std::fs::*;

pub fn input() -> String {
    let term = Term::stdout();
    term.read_line().expect("input error").trim().to_owned()
}

pub fn key_to_continue() {
    println!("Press any key to continue");
    console::Term::stdout()
        .read_key()
        .expect("key continue error");
}

pub fn load_game() {
    println!("Input the character you want to play\n(You did make a character, riiight?)..");

    let name = input();

    if !std::path::Path::new(&format!("saves/{}.txt", name)).exists() {
        println!("YOU CANNOT PLAY WITHOUT A CHARACTER YOU NOOB!");
        key_to_continue();
        return;
    }
    let mut file = File::open(format!("saves/{}.txt", name)).expect("File error");
    let player = Player::from_file(&mut file)
        .expect("Could not load character from file! Line 30 of game.rs");
    println!("{}", player);

    println!("So this game is a randomized rpg adventure game!");
    key_to_continue();
    println!("You will die and your character will be deleted on death!");
    key_to_continue();
    println!("However don't be afraid as you can always make a new one and play again and get a totally different random start!");
    key_to_continue();

    let mut level = 0i32;

    loop {
        if level == 0 {
            checkQuests(&player);
            level = 1;
        } else if level == 1 {
        }
    }
}

pub fn checkQuests(p: &Player) {
    if p.quests.is_empty() {
        println!("It appears you do not have any Quests!");
        key_to_continue();
        println!("Time to go to the cabin and pick some out!");
        key_to_continue();
    }
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

        let k = term.read_key().expect("key error 1");

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

        let input = term
            .read_char()
            .expect("read char error")
            .to_ascii_uppercase();

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

    Player::to_file(&p1);

    println!("Player file created!");

    key_to_continue();
}
