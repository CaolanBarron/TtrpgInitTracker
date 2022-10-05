use std::{fs::File, path::Path, io::{Write, stdout, BufReader, Read}, vec, fs};

use clap::{Parser};
use text_io::{try_read, read};
use crossterm::{event::{self as ct_event, KeyCode, KeyEvent, Event}, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, Clear, ClearType}, execute, cursor::MoveTo, style::{Print, SetForegroundColor, Color, ResetColor}};

use crate::character::Character;

mod character;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    file_path: String,
}

pub(crate) fn create_encounter(contents: String) {
    let mut characters: Vec<Character> = vec![];

    contents.split("-").into_iter()
        .for_each(|f| {
            if !f.is_empty() {characters.push(serde_json::from_str(f).unwrap())}
        });

    for c in characters.iter_mut() {
        if c.initiative == None {
            println!("Enter {}'s initiative: ", {&c.name()});
            c.initiative = Some(read!("{}\n"));
        }
    }
     
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
    run_encounter(characters);
    
}

pub(crate) fn create_party_file(mut file: File) {

    let mut characters = vec![];

    loop {
        println!("Enter the characters name: ");

        let name: String = read!("{}\n");

        if name.is_empty() {
            break;
        } else {
            characters.push(Character::new(name, None))
        }
    }

    characters.iter().for_each(|f| {serde_json::to_writer(&file, f).unwrap(); file.write(b"-");})
    // characters.iter().for_each(|f| file.write_all(f.as_ref().unwrap().as_bytes()).expect("Failed while writing"));
}

fn run_encounter(mut characters: Vec<Character>) {

    enable_raw_mode().unwrap();
    let mut writer = stdout();
    execute!(writer,Clear(crossterm::terminal::ClearType::All), EnterAlternateScreen).unwrap();
    let mut index: usize = 0;
    
    // Write the users options
    execute!(writer, 
        MoveTo(3, 10), 
        Print("X: Remove character"),
        MoveTo(25, 10),
        Print("<-/->: Change character"),
    );
    
    loop{
        
        // Show the previous character
        execute!(writer, 
            MoveTo(5,4),
            Clear(ClearType::CurrentLine),
            Print("Previous character: ".to_string() 
                + &characters[((index + characters.len()) - 1) % characters.len()].to_string())).unwrap();

        // Show the current character
        execute!(writer, 
            MoveTo(5,6), 
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::Green),
            Print("Current character: ".to_string() 
                + &characters[(index + characters.len()) % characters.len()].to_string()),
            ResetColor).unwrap();

        //Show the next character
        execute!(writer, 
            MoveTo(5,8), 
            Clear(ClearType::CurrentLine),
            Print("Next Character: ".to_string() 
                + &characters[((index + 1) + characters.len()) % characters.len()].to_string())).unwrap();
        
        get_input(&mut index, &mut characters);

        if characters.is_empty() {
            break;
        }
    }
    disable_raw_mode().unwrap();
    execute!(writer, Clear(ClearType::All));

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
    let arg =  Args::parse(); 
    let path = Path::new(&arg.file_path);
    match File::open(Path::new(path)) {
        Ok(f) => create_encounter(fs::read_to_string(path).unwrap()),
        Err(_) => create_party_file(File::create(Path::new(path)).unwrap()),
    };
}
