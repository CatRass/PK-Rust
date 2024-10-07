use std::fs;
use super::PkmnType::Type;

#[derive(Debug)]
pub struct Move {
    index: u16,
    typing: Type,
    name: String,
    pp: u16,
    ppup: u8
}
impl Move {
    /// Constructor for a Move, given an input move index
    pub fn get(index: u16, pp: u16, ppup: u8) -> Move {
        let moveFile = fs::read_to_string("./data/moves.pkmn").unwrap();
        let strIndex = format!("{:03}",index);
        let mut moveLine: &str = "No Move found";

        for line in moveFile.lines() {
            if line.contains(&strIndex) {
                moveLine = line;
                break;
            }
        }

        let parsedMove: Vec<&str> = moveLine.split(" ").collect();
        let name = parsedMove[1].to_string().replacen('+', " ", 1);
        let typing = Type::get(&parsedMove[2].parse::<i16>().unwrap());

        return Move{index,typing,name,pp,ppup};
    }
    /// Constructor for an empty Move slot
    pub fn empty() -> Move {
        return Move{index:0, typing: Type::Null, name: String::from("Null"), pp:0, ppup:0}
    }
    /// Returns the info on a Pokemons moves for printing
    pub fn to_string(&self) -> String {
        return format!("{} PP: {} PP Up: {}", self.name, self.pp, self.ppup);
    }

}
