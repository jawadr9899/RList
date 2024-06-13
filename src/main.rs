/*
    This CLI program is completely written in rust and its development 
    has been discontinued... due to the several issues in the language's own
    provided 'File' (API).. The functions provided in File API have plenty 
    of issues with them so the program developement has been discontiuned until 
    the 'File Package' is fully functional. 
    Since, Rust is a new programming language and it hasn't a big community
    yet! so that's why most of its provdided packages are not fully functional.
    ''' This program depdends totally upon pure rust no extra crates and the Rust's
    own File API '''
    
    @discontinued
*/
use std::fs::File;
use std::io::{stdin, stdout, BufWriter, Seek, SeekFrom};
use std::io::{Read, Write};
use std::path::Path;
use std::{fs::OpenOptions, io::Error};


#[allow(unused)]
#[derive(Debug)]
enum Modes {
    Read,
    Write,
    Append,
    ReadWrite
}
struct FileIO {
    file: File,
    mode: Modes,
}

impl FileIO {
    #[allow(unused)]
    fn new(s: &str, mode: Modes) -> Result<FileIO, Error> {
        let file = match mode {
            Modes::Read => OpenOptions::new()
                .read(true)
                .open(s)?,
            Modes::Append => OpenOptions::new()
                .append(true)
                .create(true) // Ensure file is created if it doesn't exist
                .open(s)?,
            Modes::Write => OpenOptions::new()
                .write(true)
                .create(true) // Ensure file is created if it doesn't exist
                .truncate(true)
                .open(s)?,
            Modes::ReadWrite => OpenOptions::new()
                .read(true)
                .write(true)
                .create(true) // Ensure file is created if it doesn't exist
                .open(s)?,
        };
        Ok(FileIO { file, mode })
    }
    #[allow(unused)]
    fn read(&mut self, s: &mut String) -> Result<(), Error> {
        self.file.read_to_string(s)?;
        Ok(())
    }
    #[allow(unused)]
    fn write(&mut self, cn: &str) -> Result<(), Error> {
        match self.mode {
            (Modes::Write | Modes::ReadWrite)=> {
                self.file.seek(SeekFrom::Start(0))?;  // Move cursor to the beginning
                self.file.set_len(0)?;  // Truncate the file
                let bytes = cn.as_bytes();
                let mut writer = BufWriter::new(&self.file);
                writer.write_all(bytes)?;  // Write new content
                writer.flush()?;  // Flush the buffer
                Ok(())
            },
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Arguments",
            )),
        }
    }
    #[allow(unused)]
    fn append(&mut self, cn: &str) -> Result<(), Error> {
        match self.mode {
            Modes::Append => {
                self.file.write_all(cn.as_bytes())?;
                self.file.flush()?;
                Ok(())
            }
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Arguments",
            )),
        }
    }
}
#[allow(unused)]
fn display_todo(t: &Vec<String>) {
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
#[allow(unused)]
fn get_input(s: &str) -> Result<String, Error> {
    stdout().flush().expect("failed to flush");
    println!("{}", s);
    let mut inp = String::new();
    stdin().read_line(&mut inp)?;
    Ok(inp)
}
#[allow(unused)]
fn add_todo(t: &mut Vec<String>) {
    let todo = get_input("Enter Todo:");
    let todo: String = match todo {
        Ok(t) => t.trim().to_string(),
        Err(_) => "Empty!".to_string(),
    };
    t.push(todo);

}
#[allow(unused)]

fn remove_todo(t: &mut Vec<String>,f:&mut FileIO) -> bool {
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
    // t.remove((idx-1).into());
    // remove(f, idx-1);
    t.swap_remove((idx-1).into());
    println!("Removed!");
    let mut s:String = String::new();
    for (i,n) in t.iter().enumerate() {
        s += &(n.trim().to_string() + (if i == t.len()-1 {""} else {"\n"})).to_string();
    }
    

    f.write(s.as_str()).expect("failed to write");
    true
}
#[allow(unused)]
fn remove(f: &mut FileIO,i: u8) {
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

#[allow(unused)]
fn save_todos(v: &mut Vec<String>, f: &mut FileIO, s: &mut String) -> Result<bool, Error> {
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

fn clear_screen(){
   
        // For Unix-like systems
        print!("\x1B[2J\x1B[H");
        stdout().flush().unwrap();
    
}

#[allow(unused)]
fn init(todos: &mut Vec<String>, f: &mut FileIO) {
    let mut s: String = String::new();
    loop {
        let op: String = match get_input("Inputs (+,-,view,exit,cls,save):") {
            Ok(s) => s,
            Err(_) => continue,
        };
        match op.trim() {
            "+" => add_todo(todos),
            "-" => {
                if !remove_todo(todos,f) {
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

#[allow(unused)]
fn load_into_mem(f: &mut FileIO, v: &mut Vec<String>) -> Result<bool, Error> {
    let mut s: String = String::new();
    match f.read(&mut s) {
        Ok(_) => {
            if s.is_empty() {
                return Ok(true);
            }
            s.split('\n').for_each(|n| {
                v.push(n.trim().to_string());
            });
            println!("Data loaded into memory successfully.");
            Ok(true)
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            Err(e)
        }
    }
}

fn main() {
    let mut v: Vec<String> = Vec::new();
    let mut f: FileIO;
    let file_name = "src.dat";

    let exists: bool = match Path::new("src.dat").try_exists() {
        Ok(d) => d,
        Err(e) => {
            println!("Error checking if source file exists: {}", e);
            return;
        }
    };
    
    if exists {
        f = match FileIO::new(file_name, Modes::ReadWrite) {
            Ok(file_io) => file_io,
            Err(e) => {
                println!("Failed to open existing file: {}", e);
                return;
            }
        };
    } else {
        f = match FileIO::new("src.dat", Modes::Write) {
            Ok(file_io) => file_io,
            Err(e) => {
                println!("Failed to create new file: {}", e);
                return;
            }
        };
    }

    match load_into_mem(&mut f, &mut v) {
        Ok(_) => {
            init(&mut v, &mut f);
        }
        Err(e) => {
            println!("Failed to load data into memory: {}", e);
            return;
        }
    };
}
