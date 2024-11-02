#![allow(non_snake_case)]

pub mod PKRust;
use std::{io::{self, Write}, process};

use PKRust::saveLoader::Save;

fn main() {

    io::stdout().write_all(b"Please Select a Save File: ").unwrap();
    io::stdout().flush().unwrap();

    let mut saveLocInput = String::new();
    io::stdin().read_line(&mut saveLocInput).expect("Error recieving User Input");

    // The read_line collects line endings, so we need to remove them.
    let saveLoc = saveLocInput.trim();

    // Save File must now be muttable to allow for setters
    let mut saveFile = match Save::load(saveLoc) {
        // Get the correct Save if no errors
        Ok(correctSave) => correctSave,
        // Print the error message and exit the program
        // Exits with status 1 as it is a "general error"
        Err(error) => {eprintln!("{}",error); process::exit(1);}

    };
    saveFile.print();
}
