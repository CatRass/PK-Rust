use std::fs;
use super::pokemonType::Type;

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
        let typing = Type::get(parsedMove[2].parse::<i16>().unwrap());

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

    pub fn getIndex(&self) -> &u16 {
        return &self.index;
    }

    pub fn getTyping(&self) -> &Type {
        return &self.typing;
    }

    pub fn getName(&self) -> &String {
        return &self.name;
    }

    pub fn getPP(&self) -> &u16 {
        return &self.pp;
    }

    pub fn getPPUp(&self) -> &u8 {
        return &self.ppup;
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn get_testCorrectMove() {
        let testMove:Move = Move::get(001, 3, 0);

        assert_eq!(testMove.getName(), "Pound");
        assert_eq!(testMove.getTyping(), &Type::Normal);
    }

    #[test]
    fn to_string_testCorrectStringMove() {
        let testMove:Move = Move::get(001, 5, 10);
        let stringMove: String = testMove.to_string();

        assert_eq!(stringMove, "Pound PP: 5 PP Up: 10")
    }

    #[test]
    fn get_testIncorrectMoveIndex() {
        let index = 000;

        // Assert that an error is returned
        assert!(Move::get(index, 0, 0).is_err());
        // Assert that the error is correct
        assert_eq!(Move::get(index, 0, 0), "Move with ID 000 not found.");
    }

}