#![allow(non_snake_case)]

pub mod PKRust;
use std::{io::{self, Write}, process};

use PKRust::saveLoader::Save;

fn main() {

    io::stdout().write_all(b"Please Select a Save File: ").unwrap();
    io::stdout().flush().unwrap();

    let mut saveLocInput = String::new();
    io::stdin().read_line(&mut saveLocInput).expect("Error recieving User Input");

    let mut saveLoc = "";
    // The read_line collects line endings, so we need to remove them.
    // This varies across different OS' though, so I've split it
    // into Unix and Windows
    if cfg!(windows) {
        saveLoc = saveLocInput.strip_suffix("\r\n").unwrap();
    } else if cfg!(unix) {
        saveLoc = saveLocInput.strip_suffix("\n").unwrap();
    }

    let saveFile = match Save::load(saveLoc) {
        // Get the correct Save if no errors
        Ok(correctSave) => correctSave,
        // Print the error message and exit the program
        // Exits with status 1 as it is a "general error"
        Err(error) => {eprintln!("{}",error); process::exit(1);}

    };
    saveFile.print();
}
