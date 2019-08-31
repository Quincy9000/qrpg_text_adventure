use std::fmt;
use std::fs::File;
use std::io::Read;

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
}

impl Player {
    pub fn new(name: &str, gender: Gender, age: i32) -> Player {
        Player {
            name: name.to_owned(),
            gender: gender,
            age: age,
            stats: Stats::default(),
        }
    }

    pub fn from_file(file: &mut File) -> Player {
        let mut s = String::new();
        file.read_to_string(&mut s)
            .expect("Fucking File Read Error!");
        println!("{}", s);
        let s: Vec<&str> = s.split('\n').collect();
        println!("{:?}", s);
        //todo
        let name = s.get(0);
        let p = Player {
            name,
            age: 45,
            gender: Gender::Male,
            stats: Stats::default(),
        };
        println!("\n{}", p);
        p
    }
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Physique: {}, Technique: {}, Mystique: {})",
            self.physique, self.technique, self.mystique,
        )
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Name: {}, Gender: {}, Age: {}, \nStats: {})",
            self.name, self.gender, self.age, self.stats
        )
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Other => write!(f, "Other"),
        }
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
}
