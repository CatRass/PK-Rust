#![allow(non_snake_case)]

pub mod PKRust;
use std::io::{self, Write};

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

    let saveFile = Save::load(saveLoc).unwrap();
    saveFile.print();
}
