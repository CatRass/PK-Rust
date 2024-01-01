// These values are for Gen I only!
// Pokemon Box info here: https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_Storage_System

// Pokemon Module
mod pkmod;
pub use crate::pkmod::Pokemon;
pub use crate::pkmod::PokemonSpecies    as PkmnSpecies;
pub use crate::pkmod::MovesList         as Move;

fn main() {
    println!("Hello, world!");

    let Pokemon1: Pokemon = Pokemon {
                                        species: PkmnSpecies::Ratatta,
                                        nickname: String::from("Ratatta"),
                                        moves: [Move::Tackle, Move::Tackle, Move::Tackle, Move::Tackle]
                                    };
}

struct Save {
    playerName: String,
    pokemonBox: [Box; 12],
    money: u32,
}

struct Box {
    page: [Pokemon; 240]
}