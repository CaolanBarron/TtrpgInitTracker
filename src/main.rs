use std::{fs::File, path::Path, io::{Write}, vec};
use text_io::{try_read, read};

use crate::character::Character;

mod character;

#[derive(Default)]
struct Arguments {
    file_path: String,
}

pub fn create_encounter() {
    let mut characters: Vec<Character> = vec![];

    loop {
        println!("Enter the characters name: ");
        let name:String = read!("{}\n");

        if name.is_empty() {break}

        let mut initiative:Result<u8, _>;
        loop{
            println!("Enter {}'s initiative: ", {&name});
            initiative = try_read!("{}\n");
            match initiative {
                Ok(_) => break,
                Err(_) => println!("You have not entered a number"),
            }
        }

        characters.push(Character {
            name,
            initiative: Some(initiative.unwrap()),
        })
    }

    for i in characters {
        println!("{}", i);
    }
    
}

pub fn create_party(path: &String) {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::create(path) {
        Err(why) => panic!("Couldnt write to {}: {}", display, why),
        Ok(file) => file,
    };

    let mut characters = vec![];

    loop {
        println!("Enter the characters name: ");

        let name: String = read!("{}\n");

        if name.is_empty() {
            break;
        } else {
            characters.push(serde_json::to_string(&Character::new(name, None)))
        }
    }

    characters.iter().for_each(|f| file.write_all(f.as_ref().unwrap().as_bytes()).expect("Failed while writing"));
}

fn main() {
    create_encounter();
}
