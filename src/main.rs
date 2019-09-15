mod game;
mod player;

use std::fs::DirBuilder;
use std::path::Path;

fn main() {
    let term = console::Term::stdout();
    let mut selection = 1;

    if !Path::new("saves").is_dir() {
        let db = DirBuilder::new();
        db.create("saves").expect("Fuck saves error");
    }

    loop {
        let max = 4;
        term.clear_screen().unwrap();

        println!("**********QRPG MENU**********");

        match selection {
            1 => println!("1: New Character <--\n2: Load Character\n3: Options\n4: Exit",),
            2 => println!("1: New Character\n2: Load Character <--\n3: Options\n4: Exit",),
            3 => println!("1: New Character\n2: Load Character\n3: Options <--\n4: Exit",),
            4 => println!("1: New Character\n2: Load Character\n3: Options\n4: Exit <--",),
            _ => println!("1: New Character\n2: Load Character\n3: Options\n4: Exit",),
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
                game::character_creator();
            } else if selection == 2 {
                game::load_game();
            } else if selection == 3 {
            } else if selection == 4 {
                break;
            }
        }
    }
}
