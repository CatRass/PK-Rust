// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

pub mod PKRust;

fn main() {
    let saveFile = PKRust::SaveLoader::Save::load(&std::path::PathBuf::from("./test/POKEMON YELLOW 2.sav"));
    saveFile.print();
    
}
