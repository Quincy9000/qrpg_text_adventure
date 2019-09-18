use std::fmt;
use std::fs::File;
use std::io::prelude::*;

pub trait Summarizable {
    fn summary(&self) -> String;
}

pub enum Gender {
    Male,
    Female,
    Other,
}

pub struct Stats {
    pub physique: u32,
    pub technique: u32,
    pub mystique: u32,
}

pub struct Player {
    pub name: String,
    pub gender: Gender,
    pub age: i32,
    pub stats: Stats,
    pub quests: Vec<String>,
}

impl Player {
    pub fn new(name: &str, gender: Gender, age: i32) -> Player {
        Player {
            name: name.to_string(),
            gender: gender,
            age: age,
            stats: Stats::default(),
            quests: vec!["None".to_string()],
        }
    }

    pub fn to_file(player: &Player) {
        let format = format!("saves/{}.txt", player.name);
        let mut file = File::create(format).expect("Failed to create player file.");

        let s = format!(
            "Name: {}\nAge: {}\nGender: {}\nStats: {}\nQuests: {}",
            player.name,
            player.age,
            player.gender.summary(),
            player.stats.summary(),
            player.quests.summary(),
        );

        //println!("{}", s);

        file.write_all(s.as_bytes()).expect("Fucking Error!");
    }

    pub fn from_file(file: &mut File) -> Result<Player, String> {
        let mut s = String::new();
        file.read_to_string(&mut s).expect("File Read Error!");
        // println!("{}", s);
        let vec: Vec<&str> = s.lines().collect();

        //println!("{}", vec.len());

        let name_line: Vec<&str> = vec[0].split(" ").collect();
        let name: String = name_line[1].to_owned();

        let age_line: Vec<&str> = vec[1].split(" ").collect();
        let age: i32 = age_line[1].parse().unwrap();

        let gender_line: Vec<&str> = vec[2].split(" ").collect();
        let gender: &str = gender_line[1];

        let stats: Vec<&str> = vec[3].split(|c| c == ' ' || c == ',').collect();

        // println!("{:?}", stats);
        let physique: i32 = stats[2].parse().expect("p error");
        let technique: i32 = stats[5].parse().expect("t error");
        let mystique: i32 = stats[8].replace(")", "").parse().expect("m error");

        let stats = Stats::new(physique, technique, mystique);

        let mut quest_line: Vec<&str> = vec[4].split(" ").collect();
        quest_line.remove(0);
        let mut quests: Vec<String> = Vec::new();

        for q in quest_line.iter() {
            quests.push(q.to_string());
        }

        let gender = match gender {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            "Other" => Gender::Other,
            _ => Gender::Other,
        };

        let p = Player {
            name,
            age,
            gender,
            stats,
            quests,
        };
        // println!("\n{}", p);
        Ok(p)
    }
}

impl Summarizable for Stats {
    fn summary(&self) -> String {
        format!(
            "(Physique: {}, Technique: {}, Mystique: {})",
            self.physique, self.technique, self.mystique,
        )
    }
}

impl std::fmt::Display for dyn Summarizable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl Summarizable for Player {
    fn summary(&self) -> String {
        format!(
            "Name: {}\nGender: {}\nAge: {}\nStats: {}\nQuests: {}\n",
            self.name,
            self.gender.summary(),
            self.age,
            self.stats.summary(),
            self.quests.summary(),
        )
    }
}

impl Summarizable for Gender {
    fn summary(&self) -> String {
        match self {
            Gender::Male => format!("Male"),
            Gender::Female => format!("Female"),
            Gender::Other => format!("Other"),
        }
    }
}

impl Summarizable for Vec<String> {
    fn summary(&self) -> String {
        let mut f = String::new();
        for i in self.iter() {
            f.push_str(i);
        }
        format!("{}", f)
    }
}

impl Stats {
    fn default() -> Stats {
        Stats {
            physique: 1,
            technique: 1,
            mystique: 1,
        }
    }

    fn new(physique: i32, technique: i32, mystique: i32) -> Stats {
        let physique = physique as u32;
        let technique = technique as u32;
        let mystique = mystique as u32;
        Stats {
            physique,
            technique,
            mystique,
        }
    }
}
