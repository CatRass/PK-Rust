#![allow(non_snake_case)]

pub mod PKRust;

fn main() {
    let saveFile = PKRust::SaveLoader::Save::load("./test/POKEMON YELLOW 2.sav");
    saveFile.print();
}
