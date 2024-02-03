#![allow(non_snake_case)]

use PKRust;

fn main() {

    let saveFile = PKRust::Save::load("./test/POKEMON YELLOW.sav");

    saveFile.print();

}