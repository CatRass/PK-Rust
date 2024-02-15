// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use PKRust;
use tauri::State;
use tauri::api::dialog::blocking::FileDialogBuilder;
use std::sync::Mutex;

#[tauri::command]
fn getSaveFile(saveState: State::<Mutex<Option<PKRust::Save>>>) -> String {
        
    // **TODO**: Get proper error handling on this
    let filePath = FileDialogBuilder::new()
                    .set_title("Pick a Gen 1 Save File")
                    .add_filter("Save",&["sav"])
                    .pick_file();
    
    match filePath {
        Some(path) => {    *saveState.lock().unwrap() = Some(PKRust::Save::load(&path)); 
                                    return saveState.lock().unwrap().as_ref().unwrap().to_json()
                                }
        None                => return String::from("{\"id\":-1}")
    };
}

#[tauri::command]
fn printSaveFile(saveState: State::<Mutex<Option<PKRust::Save>>>) {
    saveState.lock().unwrap().as_ref().unwrap().print();
}

fn main() {

    tauri::Builder::default()
        .manage(Mutex::new(Some(PKRust::Save::new())))
        .invoke_handler(tauri::generate_handler![
                                                    getSaveFile,
                                                    printSaveFile
                                                ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // let saveFile = PKRust::Save::load(&std::path::PathBuf::from("./test/POKEMON YELLOW 2.sav"));
    // saveFile.print();
}
