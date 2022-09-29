use std::{fs::File, path::Path, io::{Write, stdout}, vec};

use text_io::{try_read, read};
use crossterm::{event::{self as ct_event, KeyCode, KeyEvent, Event}, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen}, execute, cursor::MoveTo, style::Print};

use crate::character::Character;

mod character;

#[derive(Default)]
struct Arguments {
    file_path: String,
}

pub(crate) fn create_encounter() -> Vec<Character> {
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
    characters.sort_by(|a, b| b.initiative().cmp(&a.initiative()));
    characters
}

pub(crate) fn create_party_file(path: &String) {
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

fn run_encounter(mut characters: Vec<Character>) {

    enable_raw_mode().unwrap();
    let mut writer = stdout();
    execute!(writer, EnterAlternateScreen).unwrap();
    let mut index: usize = 0;
    
    
    loop{
        
        // Show the previous character
        execute!(writer, 
            MoveTo(5,4), 
            Print("Previous character: ".to_string() 
                + &characters[((index + characters.len()) - 1) % characters.len()].to_string())).unwrap();

        // Show the current character
        execute!(writer, 
            MoveTo(5,6), 
            Print("Current character: ".to_string() 
                + &characters[(index + characters.len()) % characters.len()].to_string())).unwrap();

        //Show the next character
        execute!(writer, 
            MoveTo(5,8), 
            Print("Next Character: ".to_string() 
                + &characters[((index + 1) + characters.len()) % characters.len()].to_string())).unwrap();
        
        get_input(&mut index, &mut characters);

        if characters.is_empty() {
            break;
        }
    }
    disable_raw_mode().unwrap();

    println!("Encounter finished!");
}

fn get_input(index: &mut usize, characters: &mut Vec<Character>) {
    let event = ct_event::read().unwrap();
    if let Event::Key(KeyEvent{
        code,
        modifiers: _,
        kind: _,
        state: _,
    }) = event {
        match code {
            KeyCode::Right => {*index += 1},
            KeyCode::Left => {*index =  ((*index + characters.len()) - 1) % characters.len()},
            KeyCode::Char('x') => {
                characters.remove((*index + characters.len()) % characters.len());
            },
            KeyCode::Esc => {characters.clear()}
            _ => (),
        }
    }
    
}
fn main() {
    
}
