// These values are for Gen I only!
// Pokemon Box info here: https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_Storage_System

#[allow(non_snake_case)]
use PKRust;

#[allow(non_snake_case)]
fn main() {

    let saveFile = PKRust::Save::load("./test/POKEMON BLUE.sav");

    saveFile.print();

}