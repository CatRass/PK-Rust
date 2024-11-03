use std::fs;
use std::path::PathBuf;

use super::creatureData::pokemonMove::Move;
use super::creatureData::pokemon::*;
use super::addresses::*;
use super::utils::{textDecode, integrityCheck, formatError};


#[derive(Debug)]
pub struct Save {
    trainer: String,
    money: u32,
    id: u16,
    party: Vec<Pokemon>,

    /// Each save file has 12 boxes, which hold 20 pokemon each.
    /// 
    /// More Info [here](https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_Storage_System)
    pc: Vec<Vec<Pokemon>>
}

#[allow(dead_code)]
impl Save {

    pub fn new() -> Save {
        return Save{    trainer: String::from("Null"),
                        money: 0,
                        id: 0,
                        party: vec![Pokemon::new()],
                        pc: Vec::new()
                    }
    } 

    pub fn load(file: &str) -> Result<Save, String>{

        let filePathBuf:PathBuf = std::path::PathBuf::from(file);

        // println!("{:?}", filePathBuf);

        // First we load the save file and check for if it exists
        // If not, an error result will be returned
        let save = match fs::read(filePathBuf) {
            Ok(result)                => result,
            Err(error)                  => match error.kind() {
                std::io::ErrorKind::NotFound   => return Err(formatError(format!("Save \"{}\" does not exist",file))),
                _                              => return Err(format!("Unexpected Error: {}",error.kind()))
            }
        };

        // Then we check if the file has integrity (Check if it's valid)
        if integrityCheck(&save) == false {
            return Err(formatError(String::from("File does not seem to be a Gen 1 Save File")));
        }

        let pc = Self::getPCBoxesFromSave(&save);

        let money = Self::getMoneyFromSave(&save);
        let id = Self::getTrainerIDFromSave(&save);
        let party:  Vec<Pokemon> = Self::getPartyFromSave(&save);
        let trainer = textDecode(&Self::getNameFromSave(&save));

        return Ok(Save{trainer, money, id, party, pc});

    }

    pub fn to_string(&self) -> String {
        return format!("{:?}",self);
    }

    /// Print the save file data to terminal
    pub fn print(&self) {
        println!("\n=== Save Info ===");
        println!("Name: {}\nPlayer ID: {}\nMoney: {}",self.trainer, self.id, self.money);
        println!("=================");

        println!("\n=== Party ===");
        for pokemon in 0..5 {
            println!("{}",&self.party[pokemon].getDetails());
        }
        println!("=================\n");
    }

    // ========   GETTERS   ========

    /// Getter for Trainer Name in Save
    pub fn getTrainerName(&self) -> &String {
        return &self.trainer;
    }    

    /// Getter for Money in Save
    pub fn getMoney(&self) -> &u32 {
        return &self.money;
    }

    /// Getter for Trainer ID in Save
    pub fn getTrainerID(&self) -> &u16 {
        return &self.id;
    }

    /// Getter for Party Pokemon
    pub fn getParty(&self) -> &Vec<Pokemon> {
        return &self.party;
    }

    /// Getter for PC Boxes
    pub fn getPCBoxes(&self) -> &Vec<Vec<Pokemon>> {
        return &self.pc;
    }

    // ========   SETTERS   ========

    /// Setter for Trainer Name in Save
    pub fn setTrainerName(&mut self, name: String) -> Result<bool, String> {

        // First let's check that the length is correct.
        if name.len() > 11 {
            return Err(formatError(format!("Name \"{}\" is over 11 characters.", name)));
        }

        // Now that the check is over, set the name
        self.trainer = name;

        // And return True
        return Ok(true);

    }

    /// Setter for Money amount in Save
    pub fn setMoney(&mut self, amount: u32) -> Result<bool, String> {

        // First let's check that the amount is correct
        // Cannot be more than 999,999
        if amount > 999_999 {
            return Err(formatError(format!("Amount \"{}\" is over allowed maximum 999,999.", amount)));
        }

        // We do not need to check for minimum as it is unsigned.

        // Now that the check is over, set the name
        self.money = amount;

        // And return True
        return Ok(true);

    }
    
    /// Setter for Trainer ID
    /// 
    /// **Note**: There is no validation in this function.
    /// This is due to the fact that the typing (`u16`) provides the ranges for the value,
    /// as [Pokemon Trainer ID's](https://bulbapedia.bulbagarden.net/wiki/Trainer_ID_number) in Gen 1
    /// cannout be *under* 0 or *over* the 16-bit unsigned integer limit.
    pub fn setID(&mut self, newID: u16) {
        
        // No checks can be done here because all is taken into account by the type.
        // Range validation should be done in the UI.
        self.id = newID;

    }

    /// Party Pokemon Setter for Nickname.
    /// 
    /// This is an abstraction for pokemon::Pokemon::setNickname
    pub fn setPartyPokemonNick(&mut self, partyPokemon: usize, newNickname: String) -> Result<bool, String> {
        
        // First we check that there is a Pokemon in the party at the index
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we update the nickname, and store the result for Error Handling
        let nicknameChangeRes = self.party[partyPokemon].setNickname(newNickname);

        // If the nickname change was unsuccessful, return the error
        if nicknameChangeRes.is_err() {
            return Err(nicknameChangeRes.unwrap_err());
        } else {
            return Ok(true);
        }
        
    }

    /// Party Pokemon Setter for Level
    /// 
    /// this is an abstraction for pokemon::Pokemon::setLevel
    pub fn setPartyPokemonLevel(&mut self, partyPokemon: usize, newLevel: i8) -> Result<bool, String> {
        todo!("Implement me")
    }

    // ========   SAVE FILE RETRIEVAL    ======== 

    /// Retrieves the name from the save file
    /// 
    /// Since most Pokemon games use character encoding, we have to decode it.
    /// 
    /// [Gen 1](https://bulbapedia.bulbagarden.net/wiki/Character_encoding_(Generation_I))
    fn getNameFromSave(save: &Vec<u8>) -> [i16;11] {
        let mut name: [i16; 11] = [0; 11];
        for num in 0..11 {
            name[num] = format!("{}",save[NAME_ADDR+num]).parse::<i16>().unwrap();
        }
        return name;
    }

    /// Retrieves the amount of money the player has
    fn getMoneyFromSave(save: &Vec<u8>) -> u32{
        return format!("{:02X}{:02X}{:02X}",save[MONEY_ADDR],save[MONEY_ADDR+1],save[MONEY_ADDR+2])
        .parse::<u32>()
        .unwrap();
    }

    /// Retrieves the trainer ID
    fn getTrainerIDFromSave(save: &Vec<u8>) -> u16 {
        let stringID = format!("{:02X}{:02X}",save[ID_ADDR],save[ID_ADDR+1]);
        return u16::from_str_radix(&stringID, 16).unwrap();
    }

    /// Retrieves the players party of Pokemon
    fn getPartyFromSave(save: &Vec<u8>) -> Vec<Pokemon> {
        let mut party:  Vec<Pokemon> = Vec::new();

        for creature in 0..save[PARTY_ADDR] as usize {
            let pkmnAddress: usize = PARTY_ADDR + 0x8 + (creature * 0x2C);
            let nickAddress: usize = PARTY_ADDR + NICK_OFF + (creature * 0xB);

            // Get current HP
            let hp = Self::getPokemonHPFromSave(&save,&pkmnAddress);
            // Nickname Obtaining code
            let nickname = Self::getPokemonNickFromSave(&save, &nickAddress);
            // Moves Obtaining code
            let moves = Self::getPokemonMovesFromSave(&save,&pkmnAddress);
            // EV Obtaining code
            let evs: [u16;5] = Self::getPokemonEVsFromSave(&save,&pkmnAddress);
            // Stat Obtaining Code
            let stats: [u16;5] = Self::getPokemonStatsFromSave(&save,&pkmnAddress);
            // IV Obtaining Code
            let ivs: [u16;5] = Self::getPokemonIVsFromSave(&save,&pkmnAddress);
            // Original Trainer Obtaining Code
            let ot = Self::getPokemonOTIDFromSave(&save,&pkmnAddress);
            let otn = Self::getPokemonOTNameFromSave(&save, &(pkmnAddress+OTN_OFF));

            party.push(Pokemon::get(    save[pkmnAddress] as i16,
                                        save[pkmnAddress+0x21] as i8,
                                        nickname,
                                        moves,
                                        ot,
                                        otn,
                                        hp, 
                                        evs, ivs, stats)
                    );

            // println!("Current Pokemon: {:#?}", party[creature]);
        }

        return party;
    }

    /// Retrieves all of the players PC boxes
    fn getPCBoxesFromSave(save: &Vec<u8>) -> Vec<Vec<Pokemon>>{
        let mut boxes: Vec<Vec<Pokemon>> = Vec::new();

        for pcBox in 0..12 as usize {
            let mut currBox: Vec<Pokemon> = Vec::new();
            // The boxes first two bytes
            let currAddr = PC_ADDR + (0x462*pcBox)%0x1A4C + (0x2000*(pcBox/6));
            let pkmnInBox = save[currAddr] as usize;
            // println!("Pokemon in box {} at {:X}: {}",pcBox+1,currAddr,pkmnInBox);

            for creature in 0..pkmnInBox {
                let pkmnAddress = currAddr + PC_PKMN_OFF + (0x21 * creature);
                let nickAddress = currAddr + PC_NICK_OFF+ (creature*0xB);

                let currSpecies: i16 = save[pkmnAddress] as i16;
                let hp = Self::getPokemonHPFromSave(&save, &pkmnAddress);
                let ot = Self::getPokemonOTIDFromSave(&save, &pkmnAddress);
                let moves = Self::getPokemonMovesFromSave(&save,&pkmnAddress);
                let nickname = Self::getPokemonNickFromSave(&save, &nickAddress);
                let evs: [u16;5] = Self::getPokemonEVsFromSave(&save,&pkmnAddress);
                let ivs: [u16;5] = Self::getPokemonIVsFromSave(&save,&pkmnAddress);
                let level: i8 = save[pkmnAddress+0x03] as i8;
                let otn = Self::getPokemonOTNameFromSave(&save, &(currAddr+PC_TRAINER_OFF+(creature*0xB)));
                // println!("Original Trainer name at: {:X}",currAddr+PC_TRAINER_OFF+(creature*0xB));

                // TODO: Finish this so it uses the box trick to calculate the proper stats.
                // https://bulbapedia.bulbagarden.net/wiki/Box_trick
                let stats: [u16;5] = [0,0,0,0,0];

                let currPkmn = Pokemon::get(currSpecies, 
                                                        level, 
                                                        nickname, 
                                                        moves, 
                                                        ot, 
                                                        otn,
                                                        hp, 
                                                        evs, 
                                                        ivs, 
                                                        stats
                                                    );
                
                currBox.push(currPkmn);

            }

            boxes.push(currBox);
        }

        // println!("{:#?}",boxes);
        return boxes;
    }

    /// Function for retrieving a Pokemons Original Trainers ID
    fn getPokemonOTIDFromSave(save: &Vec<u8>, currAddr: &usize) -> u16{
        return u16::from_str_radix( 
            &format!("{:02X}{:02X}", save[currAddr+OT_OFF],save[currAddr+OT_OFF+1]),
            16
        ).unwrap();
    }

    /// Function for retrieving a Pokemons Original Trainers Name
    /// 
    /// **Note**: There is a current bug where extra "garbage data" is added to OT Names.
    /// This most likely to do with the function textDecode note taking into account control characters.
    fn getPokemonOTNameFromSave(save: &Vec<u8>, currAddr: &usize) -> String {
        let mut encodedName: [i16;11] = [0;11];

        for char in 0..11 {
            encodedName[char] = format!("{}",save[currAddr+char]).parse::<i16>().unwrap();
        }

        return textDecode(&encodedName);
    }

    /// Function for retrieving the Pokemons current Health Points
    fn getPokemonHPFromSave(save: &Vec<u8>, currAddr: &usize) -> i16{
        return i16::from_str_radix(
            &format!("{:02X}{:02X}",save[currAddr+HP_OFF],save[currAddr+HP_OFF+1]), 
            16
            ).unwrap();
    }

    /// Function for retrieving a Pokemons Nickname.
    /// 
    /// **Note**: This function will automatically decode it into a String
    fn getPokemonNickFromSave(save: &Vec<u8>,currAddr: &usize) -> String {
        let mut encodedNick: [i16; 11]= [0; 11];
            for num in 0..11 {
                encodedNick[num] = format!("{}",save[currAddr+num]).parse::<i16>().unwrap();
            }
        return textDecode(&encodedNick);
    }

    /// Function for retrieving a Pokemons base stats
    fn getPokemonStatsFromSave(save: &Vec<u8>,currAddr: &usize) -> [u16;5] {
        let mut stats: [u16;5] = [0; 5];
            for stat in 0..5 {
                let currAddr = currAddr+STAT_OFF+(stat*2);
                stats[stat] = u16::from_str_radix(
                                                &format!("{:02X}{:02X}",save[currAddr],save[currAddr+1]),
                                                16
                                            ).unwrap();
            }
        
        return stats;
    }

    /// Function for retrieving a Pokemons Effort Values
    fn getPokemonEVsFromSave(save: &Vec<u8>,currAddr: &usize) -> [u16;5] {
        let mut evs: [u16;5] = [0; 5];
            for stat in 0..5 {
                let currAddr = currAddr+EV_OFF+(stat*2);
                evs[stat] = u16::from_str_radix(
                                                &format!("{:02X}{:02X}",save[currAddr],save[currAddr+1]),
                                                16
                                            ).unwrap();
            }
        
        return evs;
    }

    /// Function for retrieving data about Pokemons moves.
    fn getPokemonMovesFromSave(save: &Vec<u8>, currAddr: &usize) -> Vec<Move>{
    let mut returnVec: Vec<Move> = Vec::new();
    let moveAddr = currAddr + MOVE_OFF;

    for moves in 0..4 {
        let moveIndex = save[moveAddr+moves] as u16;
        let ppStr = format!("{:08b}",save[currAddr+PP_OFF+moves]);
        let (ppUp,pp) = ppStr.split_at(2);
        let currPP = u16::from_str_radix(pp, 2).unwrap();
        let currPPUp = u8::from_str_radix(ppUp, 2).unwrap();
        if moveIndex == 0 {
            returnVec.push(Move::empty());
        } else {
            returnVec.push(Move::get(moveIndex, currPP, currPPUp).unwrap());
        }
    }

    return returnVec;
}

    /// Function for retrieving a Pokemons Individual Values 
    /// (Also known as Determinant Values)
    /// 
    /// **TODO**: Figure out a better way to split into fours
    fn getPokemonIVsFromSave(save: &Vec<u8>,currAddr: &usize) -> [u16;5]{
        let allIVs = format!("{:08b}{:08b}",save[currAddr+IV_OFF],save[currAddr+IV_OFF+1]);
        let (half, half2) = allIVs.split_at(8);
        let ((atk, def), (spd, spc)) = (half.split_at(4), half2.split_at(4));
        let hp = format!("{}{}{}{}",
                                            atk.chars().last().unwrap(),
                                            def.chars().last().unwrap(),
                                            spd.chars().last().unwrap(),
                                            spc.chars().last().unwrap()
                                        );
        // println!("At address: {:#0X} IV: {}",pkmnAddress+IV_OFF,allIVs);
        let ivs: [u16;5] = [
                                u16::from_str_radix(atk,2).unwrap(),
                                u16::from_str_radix(def,2).unwrap(),
                                u16::from_str_radix(spd,2).unwrap(),
                                u16::from_str_radix(spc,2).unwrap(),
                                u16::from_str_radix(&hp,2).unwrap(),
                            ];
        
        return ivs;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_InvalidFile() {
        let fileName = "./test/POKEMON CRYSTAL.sav";
        let saveFile = Save::load(fileName);

        assert!(saveFile.is_err());
        assert_eq!(saveFile.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: File does not seem to be a Gen 1 Save File");
        
    }

    #[test]
    fn load_ValidFile() {
        let fileName = "./test/POKEMON BLUE.sav";
        let saveFile = Save::load(fileName);

        assert!(saveFile.is_ok());
    }

    #[test]
    fn load_NonexistentFile() {
        let fileName = "./test/Nonexistent File.sav";
        let saveFile = Save::load(fileName);

        assert!(saveFile.is_err());
        assert_eq!(saveFile.unwrap_err(), format!("\u{1b}[0;31mError\u{1b}[0m: Save \"{}\" does not exist",fileName));
        
    }

    #[test]
    fn setTrainerName_Correct() {
        // This is the save we'll be testing with
        let mut currSave = Save::new();

        // This is the trainer name we'll be setting
        let newName = String::from("Brock");

        // Now we change the name
        let nameChangeResult = currSave.setTrainerName(newName);

        // Once Gotten, we expect it to be Ok() and not Err()
        assert!(nameChangeResult.is_ok());
        // And we expect the unwrapped version of the name to be true
        assert_eq!(nameChangeResult.unwrap(), true);
    }

    #[test]
    fn setTrainerName_Inorrect() {
        // This is the save we'll be testing with
        let mut currSave = Save::new();

        // This is the trainer name we'll be setting.
        // The maximum length for a name in gen 1 is 11 chars.
        // Thus we will be testing with more than 11
        let newName = String::from("Professor Oak");

        // Now we change the name
        let nameChangeResult = currSave.setTrainerName(newName);

        // Once Gotten, we expect it to be Ok() and not Err()
        assert!(nameChangeResult.is_err());
        // And we expect the unwrapped version of the name to be true
        assert_eq!(nameChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Name \"Professor Oak\" is over 11 characters.");
    }

    #[test]
    fn setMoney_Correct() {
        let mut testSave = Save::new();

        let newMoneyAmount:u32 = 1000;

        let moneyChangeResult = testSave.setMoney(newMoneyAmount);

        assert!(moneyChangeResult.is_ok());
        assert_eq!(moneyChangeResult.unwrap(), true);
    }

    #[test]
    fn setMoney_Incorrect() {
        let mut testSave = Save::new();

        let newMoneyAmount:u32 = 1_000_000;

        let moneyChangeResult = testSave.setMoney(newMoneyAmount);

        assert!(moneyChangeResult.is_err());
        assert_eq!(moneyChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Amount \"1000000\" is over allowed maximum 999,999.") 
    }

    #[test]
    fn setID_Correct() {
        let mut testSave = Save::new();

        let newID:u16 = 12345;

        testSave.setID(newID);

        assert_eq!(testSave.getTrainerID(), &newID);
    }

    #[test]
    fn setPartyPokemonNick_CorrectIndex() {
        let mut testSave = Save::new();

        let newNickname = String::from("Jimsaur");

        let nicknameChangeResult = testSave.setPartyPokemonNick(0, newNickname);

        assert!(nicknameChangeResult.is_ok());
        assert_eq!(nicknameChangeResult.unwrap(), true);
    }

    #[test]
    fn setPartyPokemonNick_IncorrectIndex() {
        let mut testSave = Save::new();

        let newNickname = String::from("Charmander");

        // We'll try to edit a pokemon that doesn't exist
        let nicknameChangeResult = testSave.setPartyPokemonNick(1, newNickname);

        assert!(nicknameChangeResult.is_err());
        assert_eq!(nicknameChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1");
    }

    #[test]
    fn setPartyPokemonNick_CorrectLength() {
        let mut testSave = Save::new();

        // Length 11, boundary value
        let newNickname = String::from("Jimbosaurus");

        let nicknameChangeResult = testSave.setPartyPokemonNick(0, newNickname);

        assert!(nicknameChangeResult.is_ok());
        assert_eq!(nicknameChangeResult.unwrap(), true);
    }

    #[test]
    fn setPartyPokemonNick_IncorrectLength() {
        let mut testSave = Save::new();

        // Length 15, too high
        let newNickname = String::from("Jimbosaurus Rex");

        let nicknameChangeResult = testSave.setPartyPokemonNick(0, newNickname);

        assert!(nicknameChangeResult.is_err());
        assert_eq!(nicknameChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Length of nickname \"Jimbosaurus Rex\" is over 11 characters.");
    }

    #[test]
    fn setPartyPokemonLevel_Correct() {
        let mut testSave = Save::new();

        let newLevel:i8 = 100;

        let levelChangeResult = testSave.setPartyPokemonLevel(0, newLevel);

        assert!(levelChangeResult.is_ok());
        assert_eq!(levelChangeResult.unwrap(), true); 
    }

    #[test]
    fn setPartyPokemonLevel_InorrectIndex() {
        let mut testSave = Save::new();

        let newLevel:i8 = 100;

        let levelChangeResult = testSave.setPartyPokemonLevel(1, newLevel);

        assert!(levelChangeResult.is_err());
        assert_eq!(levelChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1"); 
    }

    #[test]
    fn setPartyPokemonLevel_IncorrectUnder() {
        let mut testSave = Save::new();

        let newLevel:i8 = 0;

        let levelChangeResult = testSave.setPartyPokemonLevel(0, newLevel);

        assert!(levelChangeResult.is_err());
        assert_eq!(levelChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Value of new level \"0\" is under allowed minimum 1"); 
    }

    #[test]
    fn setPartyPokemonLevel_IncorrectOver() {
        let mut testSave = Save::new();

        let newLevel:i8 = 101;

        let levelChangeResult = testSave.setPartyPokemonLevel(0, newLevel);

        assert!(levelChangeResult.is_err());
        assert_eq!(levelChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Value of new level \"101\" is over allowed maximum 100"); 
    }

}