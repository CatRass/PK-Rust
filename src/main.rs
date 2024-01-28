#![allow(non_snake_case)]
// These values are for Gen I only!
// Pokemon Box info here: https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_Storage_System

use PKRust;

fn main() {

    let saveFile = PKRust::Save::load("./test/POKEMON YELLOW.sav");

    saveFile.print();

}