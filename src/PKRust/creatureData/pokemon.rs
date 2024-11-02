use super::pokemonMove::Move;
use super::pokemonSpecies::Species;
use super::pokemonStats::{Stats, IVs, EVs};


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

        let evs = EVs{hp: evArr[0], atk: evArr[1], def: evArr[2], spd: evArr[3], spc: evArr[4]};
        let ivs = IVs{atk: ivArr[0], def: ivArr[1], spd: ivArr[2], spc: ivArr[3], hp: ivArr[4]};
        let stats = Stats{hp: statArr[0], atk: statArr[1], def: statArr[2], spd: statArr[3], spc: statArr[4]};
        
        return Pokemon{nickname, species, level, moves, ot, otn, hp, evs, ivs, stats};
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

    pub fn getOT(&self) -> &u16 {
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

    pub fn setNickname(&mut self, newNickname: String) -> Result<bool, String> {
        todo!("Implement me");
    }
    

}



#[cfg(test)]
mod tests {
    // use crate::pkmnLib::utils::textDecode;

    use std::vec;

    use super::*;

    #[test]
    fn getDetails() {
        // Test Pokemon is a Bulbasaur at level 10
        let testPkmn:Pokemon = Pokemon::get(
            0x99,
            10,
            "Testy".to_string(),
            vec![Move::empty(), Move::empty(), Move::empty(), Move::empty()],
            2, "Test Ketchum".to_string(),
            100,
            [0;5],
            [0;5],
            [0;5]
        );

        let stringPokemon:String = testPkmn.getDetails();
        let actualString = "Bulbasaur    Testy        LVL:10 Current HP: 100\n\tNull PP: 0 PP Up: 0\n\tNull PP: 0 PP Up: 0\n\tNull PP: 0 PP Up: 0\n\tNull PP: 0 PP Up: 0\n\n\n\tHP: 0\n\tATK: 0\n\tDEF:0\n\tSPD: 0\n\tSPCL: 0\n\n\tHP EV: 0\n\tATK EV: 0\n\tDEF EV:0\n\tSPD EV: 0\n\tSPCL EV: 0\n\n\tATK IV: 0\n\tDEF IV:0\n\tSPD IV: 0\n\tSPCL IV: 0\n";
        assert_eq!(stringPokemon, actualString);
    }

    #[test]
    fn setNickname_Correct() {
        let mut testPokemon = Pokemon::get(
            0x99, 
            1, 
            String::from("Bobsaur"), 
            vec![], 
            1, 
            String::from("Ash Ketchup"), 
            50, 
            [1,1,1,1,1], [1,1,1,1,1], [1,1,1,1,1]
        );

        let newNickname = String::from("Jimsaur");

        let nicknameResult = testPokemon.setNickname(newNickname);

        assert!(nicknameResult.is_ok());
        assert_eq!(nicknameResult.unwrap(), true);

    }

    #[test]
    fn setNickname_Inorrect() {
        let mut testPokemon = Pokemon::get(
            0x99, 
            1, 
            String::from("Bobsaur"), 
            vec![], 
            1, 
            String::from("Ash Ketchup"), 
            50, 
            [1,1,1,1,1], [1,1,1,1,1], [1,1,1,1,1]
        );

        let newNickname = String::from("Jimbosaurus Rex");

        let nicknameResult = testPokemon.setNickname(newNickname);

        assert!(nicknameResult.is_err());
        assert_eq!(nicknameResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Length of nickname \"Jimbosaurus Rex\" is over 11 characters.");

    }

}