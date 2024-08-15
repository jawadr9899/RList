use crate::fileio::{FileIO, Modes};
use std::io::Write;
use std::io::{stdin, stdout, Error};
use std::path::Path;
use std::process::exit;

pub fn display_todo(t: &Vec<String>) {
    if t.is_empty() {
        println!("| --- Empty --- |");
        return;
    }
    const XTRA_SPACE: u8 = 3;
    for (i, j) in t.iter().enumerate() {
        println!("{}| {} |", i + 1, j.trim());
        for _ in 0..=j.len() as u8 + XTRA_SPACE {
            print!("-");
        }
        println!();
    }
}

pub fn get_input(s: &str) -> Result<String, Error> {
    stdout().flush().expect("failed to flush");
    println!("{}", s);
    let mut inp = String::new();
    stdin().read_line(&mut inp)?;
    Ok(inp)
}

pub fn add_todo(t: &mut Vec<String>) {
    let todo = get_input("Enter Todo:");
    let todo: String = match todo {
        Ok(t) => t.trim().to_string(),
        Err(_) => "Empty!".to_string(),
    };
    t.push(todo);
}

pub fn remove_todo(t: &mut Vec<String>, f: &mut FileIO) -> bool {
    let idx = get_input("Enter Index:");

    let idx: u8 = match idx {
        Ok(i) => i.trim().parse::<u8>().expect("failed to parse"),
        Err(_) => {
            return false;
        }
    };
    if t.len() == 0 {
        println!("Empty list!");
        return false;
    }
    if idx == 0 || idx > t.len() as u8 {
        println!("Invalid Index!");
        return false;
    }
    t.swap_remove((idx - 1).into());
    println!("Removed!");
    let mut s: String = String::new();
    for (i, n) in t.iter().enumerate() {
        s += &(n.trim().to_string() + (if i == t.len() - 1 { "" } else { "\n" })).to_string();
    }

    f.write(s.as_str()).expect("failed to write");
    true
}

pub fn remove(f: &mut FileIO, i: u8) {
    let mut readable: String = String::new();
    f.read(&mut readable).expect("failed to read!");
    let mut v: Vec<String> = readable
        .trim()
        .split("\n")
        .map(|x| x.trim().to_owned())
        .collect();
    readable.clear();
    v.swap_remove(i.into());
    for i in &mut v {
        i.push('\n');
        readable += i;
    }
    readable = readable.trim().to_string();
    let mut f = FileIO::new("index.txt", Modes::Write).expect("Failed to open file!");
    f.write(&readable).expect("failed to write");
}

pub fn save_todos(v: &mut Vec<String>, f: &mut FileIO, s: &mut String) -> Result<bool, Error> {
    if v.len() == 0 {
        return Ok(true);
    };
    *s = String::new();
    for n in &mut *v {
        *n = n.trim().to_string();
        n.push('\n');
        *s += n;
    }
    println!("Saved!");
    f.write(s.trim())?;
    Ok(true)
}

pub fn clear_screen() {
    // For Unix-like systems
    print!("\x1B[2J\x1B[H");
    stdout().flush().unwrap();
}

pub fn init(todos: &mut Vec<String>, f: &mut FileIO) {
    let mut s: String = String::new();
    loop {
        let op: String = match get_input("Inputs (+,-,view,exit,cls,save):") {
            Ok(s) => s,
            Err(_) => continue,
        };
        match op.trim() {
            "+" => add_todo(todos),
            "-" => {
                if !remove_todo(todos, f) {
                    continue;
                }
            }
            "view" => {
                println!();
                display_todo(&todos);
            }
            "exit" => {
                println!("Thanks for using!");
                break;
            }
            "cls" => clear_screen(),
            "save" => {
                save_todos(todos, f, &mut s);
            }
            _ => {
                println!("Invalid input!");
                continue;
            }
        }
    }
}

pub fn load_into_mem(f: &mut FileIO, v: &mut Vec<String>) -> Result<bool, Error> {
    let mut s: String = String::new();
    match f.read(&mut s) {
        Ok(_) => {
            if s.is_empty() {
                return Ok(true);
            }
            s.split('\n').for_each(|n| {
                v.push(n.trim().to_string());
            });
            println!("Loaded file \"{}\"", f.filename());
            Ok(true)
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            Err(e)
        }
    }
}
