#![allow(non_snake_case)]

pub mod PKRust;

fn main() {
    let saveFile = PKRust::saveLoader::Save::load("./test/POKEMON YELLOW 2.sav").unwrap();
    saveFile.print();
}
