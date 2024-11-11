use super::pokemonMove::Move;
use super::pokemonSpecies::Species;
use super::pokemonStats::{Stats, IVs, EVs};
use super::super::utils::formatError;


#[derive(Debug)]
pub struct Pokemon {
    nickname:   String,
    species:    Species,
    level:      i8,
    moves:      Vec<Move>,
    /// Original Trainer ID
    ot:         u16,
    /// Original Trainer Name
    otn:        String,
    hp:         i16,
    evs:        EVs,
    ivs:        IVs,
    stats:      Stats,
}
impl Pokemon {

    /// Constructor for a Pokemon, when being read from a save file
    pub fn get(index: i16, level:i8, nickname: String, moves: Vec<Move>, ot: u16, otn: String, hp: i16, evArr: [u16;5], ivArr: [u16;5], statArr: [u16;5]) -> Pokemon {
        let species = Species::parse(index).unwrap();

        let evs = EVs::setAll(evArr);
        let ivs = IVs::setAll(ivArr);
        let stats = Stats{hp: statArr[0], atk: statArr[1], def: statArr[2], spd: statArr[3], spc: statArr[4]};
        
        return Pokemon{nickname, species, level, moves, ot, otn, hp, evs, ivs, stats};
    }

    /// Function for making a blank Pokemon
    pub fn new() -> Pokemon {
        return Pokemon::get(
            0x99,
            10,
            "Bobsaur".to_string(),
            vec![Move::empty(), Move::empty(), Move::empty(), Move::empty()],
            2, "Test Ketchum".to_string(),
            100,
            [0;5],
            [0;5],
            [0;5]
        );
    }

    // ========   GETTERS   ========

    /// Returns a string with all of the Pokemon's details, such as:
    /// 
    /// - Species
    /// - Nickname
    /// - Level
    /// - Current HP
    /// - EVs
    /// - IVs
    /// - Stats
    pub fn getDetails(&self) -> String{
        let basicDetails= format!("{:12} {:12} LVL:{} Current HP: {}\n",
                                        self.species.getName(), 
                                        self.nickname, 
                                        self.level,
                                        self.hp
                                    ); 
        let evDetails   = self.evs.to_string();
        let ivDetails   = self.ivs.to_string();
        let statDetails   = self.stats.to_string();

        let moves: Vec<String> =  vec![
                                            self.moves[0].to_string(),
                                            self.moves[1].to_string(),
                                            self.moves[2].to_string(),
                                            self.moves[3].to_string(),
                                        ];
        let moveDetails = format!("\t{}\n\t{}\n\t{}\n\t{}\n\n",moves[0], moves[1], moves[2], moves[3]);

        return format!("{}{}\n{}\n{}\n{}", basicDetails, moveDetails, statDetails, evDetails, ivDetails);
    }

    pub fn getNickname(&self) -> &String {
        return &self.nickname;
    }

    pub fn getSpecies(&self) -> &Species {
        return &self.species;
    }

    pub fn getLevel(&self) -> &i8 {
        return &self.level;
    }

    pub fn getMoves(&self) -> &Vec<Move> {
        return &self.moves;
    }

    pub fn getOTID(&self) -> &u16 {
        return &self.ot;
    }

    pub fn getOTN(&self) -> &String {
        return &self.otn;
    }

    pub fn getHP(&self) -> &i16 {
        return &self.hp;
    }

    pub fn getEVs(&self) -> &EVs {
        return &self.evs;
    }

    pub fn getIVs(&self) -> &IVs {
        return &self.ivs;
    }

    pub fn getStats(&self) -> &Stats {
        return &self.stats;
    }

    // ========   SETTERS   ========

    /// Setter for Pokemon Nickname
    pub fn setNickname(&mut self, newNickname: String) -> Result<bool, String> {

        // First we check if the nickname is over 11 chars
        if newNickname.len() > 10 {
            return Err(formatError(format!("Length of nickname \"{}\" is over 10 characters.", newNickname)));
        }

        // Now that the check is complete, we change the nickname
        self.nickname = newNickname;

        return Ok(true);
    }
    
    /// Setter Pokemon Level
    pub fn setLevel(&mut self, newLevel:i8) -> Result<bool, String> {
        
        // First we check that the level is not over 100
        // Then we check if it is under 1
        if newLevel > 100 {
            return Err(formatError(format!("Value of new level \"{}\" is over allowed maximum 100", newLevel)));
        } else if newLevel < 1 {
            return Err(formatError(format!("Value of new level \"{}\" is under allowed minimum 1", newLevel)));
        }

        // Now that the checks are complete, we can set the new level
        self.level = newLevel;

        // And return an Ok()
        return Ok(true);
    }

    /// Setter for Pokemon OT ID
    /// 
    /// No validation is required in the function as the
    /// typing restrictions handle the minimum and maximum values.
    /// 
    /// Any range validation needs to be done on user input.
    pub fn setOTID(&mut self, newOTID:u16) {
        self.ot = newOTID;
    }


    /// Setter for Pokemon OT Nickname
    pub fn setOTN(&mut self, newOTN:String) -> Result<bool, String>{
        
        // First we check that the length is correct
        if newOTN.len() > 10 {
            return Err(formatError(format!("OTN \"{}\" is over 7 characters.", newOTN)));
        }

        // Now that we've checked the length, we set the name
        self.otn = newOTN;

        // and return an Ok
        return Ok(true);
    }
}



#[cfg(test)]
mod tests {
    use std::u16;

    use super::*;

    #[test]
    fn getDetails() {
        // Test Pokemon is a Bulbasaur at level 10
        let testPkmn:Pokemon = Pokemon::new();

        let stringPokemon:String = testPkmn.getDetails();
        let actualString = "Bulbasaur    Bobsaur      LVL:10 Current HP: 100\n\tNull PP: 0 PP Up: 0\n\tNull PP: 0 PP Up: 0\n\tNull PP: 0 PP Up: 0\n\tNull PP: 0 PP Up: 0\n\n\n\tHP: 0\n\tATK: 0\n\tDEF:0\n\tSPD: 0\n\tSPCL: 0\n\n\tHP EV: 0\n\tATK EV: 0\n\tDEF EV:0\n\tSPD EV: 0\n\tSPCL EV: 0\n\n\tATK IV: 0\n\tDEF IV:0\n\tSPD IV: 0\n\tSPCL IV: 0\n";
        assert_eq!(stringPokemon, actualString);
    }

    #[test]
    fn setNickname_Correct() {
        let mut testPokemon = Pokemon::new();

        let newNickname = String::from("Jimsaur");

        let nicknameResult = testPokemon.setNickname(newNickname);

        assert!(nicknameResult.is_ok());
        assert_eq!(nicknameResult.unwrap(), true);

    }

    #[test]
    fn setNickname_Incorrect() {
        let mut testPokemon = Pokemon::new();

        let newNickname = String::from("Jimbosaurus Rex");

        let nicknameResult = testPokemon.setNickname(newNickname);

        assert!(nicknameResult.is_err());
        assert_eq!(nicknameResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Length of nickname \"Jimbosaurus Rex\" is over 10 characters.");

    }

    #[test]
    fn setLevel_Correct() {
        // Test Pokemon
        let mut testPokemon = Pokemon::new();

        let newLevel:i8 = 100;

        let levelChangeResult = testPokemon.setLevel(newLevel);

        assert!(levelChangeResult.is_ok());
        assert_eq!(levelChangeResult.unwrap(), true);
    }

    #[test]
    fn setLevel_IncorrectOver() {
        // Test Pokemon
        let mut testPokemon = Pokemon::new();

        // Max level can only be 100
        let newLevel:i8 = 101;

        let levelChangeResult = testPokemon.setLevel(newLevel);

        assert!(levelChangeResult.is_err());
        assert_eq!(levelChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Value of new level \"101\" is over allowed maximum 100");
    }

    #[test]
    fn setLevel_IncorrectUnder() {
        // Test Pokemon
        let mut testPokemon = Pokemon::new();

        // Max level can only be 100
        let newLevel:i8 = 0;

        let levelChangeResult = testPokemon.setLevel(newLevel);

        assert!(levelChangeResult.is_err());
        assert_eq!(levelChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Value of new level \"0\" is under allowed minimum 1");
    }

    #[test]
    fn setOTID_Correct() {

        let mut testPokemon = Pokemon::new();

        let newOTID = u16::MAX;

        testPokemon.setOTID(newOTID);

        assert_eq!(testPokemon.getOTID(), &newOTID);

    }

    #[test]
    fn setOTN_Correct() {
        let mut testPkmn: Pokemon = Pokemon::new();
        let newOTN: String = String::from("Brock");

        let changeOTNResult = testPkmn.setOTN(newOTN);

        assert!(changeOTNResult.is_ok());
        assert_eq!(changeOTNResult.unwrap(), true);
    }

    #[test]
    fn setOTN_Incorrect() {
        let mut testPkmn: Pokemon = Pokemon::new();

        // 13 Char OTN, can only be 7
        let newOTN: String = String::from("Professor Oak");
        
        let changeOTNResult = testPkmn.setOTN(newOTN);

        assert!(changeOTNResult.is_err());
        assert_eq!(changeOTNResult.unwrap_err(),"\u{1b}[0;31mError\u{1b}[0m: OTN \"Professor Oak\" is over 7 characters.");
    }

}