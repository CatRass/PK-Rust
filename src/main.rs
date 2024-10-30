#![allow(non_snake_case)]

pub mod PKRust;
use PKRust::saveLoader::Save;

fn main() {
    let saveFile = Save::load("./test/POKEMON YELLOW 2.sav").unwrap();
    saveFile.print();
}
