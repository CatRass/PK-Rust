// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use pkmnLib as PKRust;
pub mod pkmnLib;

fn main() {

    let saveFile = PKRust::Save::load(&std::path::PathBuf::from("./test/POKEMON YELLOW 2.sav"));
    saveFile.print();
    
}
