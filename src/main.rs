#![allow(unused)]
use std::fs;
use std::io::Write;
use std::io::{stdin, stdout, Error};
use std::path::Path;
use std::process::exit;
pub mod fileio;
pub mod utils;
use fileio::{FileIO, Modes};
use utils::{init, load_into_mem};

fn main() {
    let mut v: Vec<String> = Vec::new();
    let mut f: FileIO;
    let file_name = "src.dat";

    let exists: bool = match Path::new(file_name).try_exists() {
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
        println!("File not found! Creating...");
        if let Ok(f) = fs::File::create(file_name) {
            drop(f);
        } else {
            println!("Some Error Occured while creating file!");
            exit(0);
        }

        f = match FileIO::new(file_name, Modes::Write) {
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
