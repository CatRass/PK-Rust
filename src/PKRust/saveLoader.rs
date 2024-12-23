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
        if name.len() > 7 {
            return Err(formatError(format!("Name \"{}\" is over 7 characters.", name)));
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
    /// This is an abstraction for pokemon::Pokemon::setLevel
    pub fn setPartyPokemonLevel(&mut self, partyPokemon: usize, newLevel: i8) -> Result<bool, String> {
        
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we update the pokemons level and store the result
        let levelChangeResult = self.party[partyPokemon].setLevel(newLevel);
    
        if levelChangeResult.is_err() {
            return Err(levelChangeResult.unwrap_err());
        } else {
            return Ok(true);
        }
    }

    /// Party Pokemon Setter for OTID
    /// 
    /// This is an abstraction for pokemon::Pokemon::setOTID
    /// 
    /// **Note**: There is no range validation for the OTID in this function.
    /// This is due to the fact that the typing (`u16`) provides the ranges for the value,
    /// as [Pokemon Trainer ID's](https://bulbapedia.bulbagarden.net/wiki/Trainer_ID_number) in Gen 1
    /// cannout be *under* 0 or *over* the 16-bit unsigned integer limit.
    pub fn setPartyPokemonOTID(&mut self, partyPokemon: usize, newOTID: u16) -> Result<bool, String> {

         // First we check that the party index is existing
         if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we edit the OT ID
        self.party[partyPokemon].setOTID(newOTID);

        // And finally we return a successful result
        return Ok(true);
    }

    /// Party Pokemon Setter for OT Nickname
    /// 
    /// This is an abstraction for pokemon::Pokemon::setOTN
    pub fn setPartyPokemonOTN(&mut self, partyPokemon: usize, newOTN: String) -> Result<bool, String> {
        
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we edit the OTN, and store the result
        let changeOTNResult = self.party[partyPokemon].setOTN(newOTN);

        // If we get an error, we return it
        // Else, return a success
        if changeOTNResult.is_err() {
            return Err(changeOTNResult.unwrap_err());
        } else {
            return Ok(true);
        }

    }

    /// Party Pokemon Setter for HP EV
    /// 
    /// Abstraction for pokemon::Pokemon::setEV_HP()
    pub fn setPartyPokemonEV_HP(&mut self, partyPokemon: usize, newHP: u16) -> Result<bool, String> {
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we perform the change (no need to store the result, as EVs don't return Result objects)
        self.party[partyPokemon].setEV_HP(newHP);

        // And return a success
        return Ok(true);
    } 

    /// Party Pokemon Setter for ATK EV
    /// 
    /// Abstraction for pokemon::Pokemon::setEV_ATK()
    pub fn setPartyPokemonEV_ATK(&mut self, partyPokemon: usize, newATK: u16) -> Result<bool, String> {
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we perform the change (no need to store the result, as EVs don't return Result objects)
        self.party[partyPokemon].setEV_ATK(newATK);

        // And return a success
        return Ok(true);
    } 

    /// Party Pokemon Setter for DEF EV
    /// 
    /// Abstraction for pokemon::Pokemon::setEV_DEF()
    pub fn setPartyPokemonEV_DEF(&mut self, partyPokemon: usize, newDEF: u16) -> Result<bool, String> {
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we perform the change (no need to store the result, as EVs don't return Result objects)
        self.party[partyPokemon].setEV_DEF(newDEF);

        // And return a success
        return Ok(true);
    } 

    /// Party Pokemon Setter for SPD EV
    /// 
    /// Abstraction for pokemon::Pokemon::setEV_SPD()
    pub fn setPartyPokemonEV_SPD(&mut self, partyPokemon: usize, newSPD: u16) -> Result<bool, String> {
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we perform the change (no need to store the result, as EVs don't return Result objects)
        self.party[partyPokemon].setEV_SPD(newSPD);

        // And return a success
        return Ok(true);
    } 

    /// Party Pokemon Setter for SPC EV
    /// 
    /// Abstraction for pokemon::Pokemon::setEV_SPC()
    pub fn setPartyPokemonEV_SPC(&mut self, partyPokemon: usize, newSPC: u16) -> Result<bool, String> {
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we perform the change (no need to store the result, as EVs don't return Result objects)
        self.party[partyPokemon].setEV_SPC(newSPC);

        // And return a success
        return Ok(true);
    } 

    /// Party Pokemon Setter for HP IV
    /// 
    /// Abstraction for pokemon::Pokemon::setIV_HP()
    pub fn setPartyPokemonIV_HP(&mut self, partyPokemon: usize, newHP: u16) -> Result<bool, String> {
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we perform the change (no need to store the result, as EVs don't return Result objects)
        let changeHPResult = self.party[partyPokemon].setIV_HP(newHP);

        // And return a success
        return changeHPResult;
    } 

    /// Party Pokemon Setter for ATK IV
    /// 
    /// Abstraction for pokemon::Pokemon::setIV_ATK()
    pub fn setPartyPokemonIV_ATK(&mut self, partyPokemon: usize, newATK: u16) -> Result<bool, String> {
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we perform the change (no need to store the result, as EVs don't return Result objects)
        let changeATKResult = self.party[partyPokemon].setIV_ATK(newATK);

        // And return a success
        return changeATKResult;
    } 

    /// Party Pokemon Setter for DEF IV
    /// 
    /// Abstraction for pokemon::Pokemon::setIV_DEF()
    pub fn setPartyPokemonIV_DEF(&mut self, partyPokemon: usize, newDEF: u16) -> Result<bool, String> {
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we perform the change (no need to store the result, as EVs don't return Result objects)
        let changeDEFResult = self.party[partyPokemon].setIV_DEF(newDEF);

        // And return a success
        return changeDEFResult;
    } 

    /// Party Pokemon Setter for SPD IV
    /// 
    /// Abstraction for pokemon::Pokemon::setIV_SPD()
    pub fn setPartyPokemonIV_SPD(&mut self, partyPokemon: usize, newSPD: u16) -> Result<bool, String> {
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we perform the change (no need to store the result, as EVs don't return Result objects)
        let changeSPDResult = self.party[partyPokemon].setIV_SPD(newSPD);

        // And return a success
        return changeSPDResult;
    } 

    /// Party Pokemon Setter for SPC IV
    /// 
    /// Abstraction for pokemon::Pokemon::setIV_SPC()
    pub fn setPartyPokemonIV_SPC(&mut self, partyPokemon: usize, newSPC: u16) -> Result<bool, String> {
        // First we check that the party index is existing
        if partyPokemon > self.party.len() - 1 {
            return Err(formatError(format!("There is no Pokemon in party slot {}", partyPokemon)));
        }

        // Then we perform the change (no need to store the result, as EVs don't return Result objects)
        let changeSPCResult = self.party[partyPokemon].setIV_SPC(newSPC);

        // And return a success
        return changeSPCResult;
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
    /// TODO: Figure out how to do this without using strings
    fn getMoneyFromSave(save: &Vec<u8>) -> u32{
        
        // let mut moneyInt:u32 = 0b0;
        // for moneyOffset in 0..3 {
        //     moneyInt |= save[MONEY_ADDR+moneyOffset] as u32;
        //     moneyInt <<= 8;
        // }

        // println!("Supposed to be: {0:08b}{:08b}{:08b}{:08b}", save[MONEY_ADDR],save[MONEY_ADDR+1],save[MONEY_ADDR+2]);
        // println!("Is: {:032b}", moneyInt);

        // return moneyInt

        return format!("{:02X}{:02X}{:02X}",save[MONEY_ADDR],save[MONEY_ADDR+1],save[MONEY_ADDR+2])
        .parse::<u32>()
        .unwrap();
        
    }

    /// Retrieves the trainer ID
    fn getTrainerIDFromSave(save: &Vec<u8>) -> u16 {

        let mut trainerID:u16 = 0b0;

        trainerID |= (save[ID_ADDR] as u16) << 8;
        trainerID |= save[ID_ADDR+1] as u16;

        return trainerID;
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
        let mut pokemonOTID:u16 = 0b0;

        pokemonOTID |= (save[currAddr+OT_OFF] as u16) << 8;
        pokemonOTID |= save[currAddr+OT_OFF+1] as u16;

        return pokemonOTID;
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

        let mut pokemonHP:i16 = 0x0;

        pokemonHP |=  (save[currAddr+HP_OFF] as i16) << 8;
        pokemonHP |=  save[currAddr+HP_OFF + 1] as i16;

        return  pokemonHP;
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
        let mut stats: [u16;5] = [0x0; 5];
        
        for stat in 0..5 {
            let currAddr = currAddr+STAT_OFF+(stat*2);
            stats[stat] |= (save[currAddr] as u16) << 8;
            stats[stat] |= save[currAddr + 1] as u16;
        }
        
        return stats;
    }

    /// Function for retrieving a Pokemons Effort Values
    fn getPokemonEVsFromSave(save: &Vec<u8>,currAddr: &usize) -> [u16;5] {
        let mut evs: [u16;5] = [0x0; 5];

        for stat in 0..5 {
            let currAddr = currAddr+EV_OFF+(stat*2);
            evs[stat] |= (save[currAddr] as u16) << 8;
            evs[stat] |= save[currAddr + 1] as u16;
        }
        
        return evs;
    }

    /// Function for retrieving data about Pokemons moves.
    fn getPokemonMovesFromSave(save: &Vec<u8>, currAddr: &usize) -> Vec<Move>{
    let mut returnVec: Vec<Move> = Vec::new();
    let moveAddr = currAddr + MOVE_OFF;

    for moves in 0..4 {
        let moveIndex = save[moveAddr+moves] as u16;

        // As per the Wikiw (https://m.bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_data_structure_(Generation_I)#PP), PP values are determined by:
        // The current PP-Up being the first two bits of the move
        // The current PP value being the remaining 6 bits
        let ppValues = save[currAddr+PP_OFF+moves];

        let PP: u16 = ((ppValues << 2) >> 2) as u16 ;
        let PPUp: u8 = ppValues >> 6;

        if moveIndex == 0 {
            returnVec.push(Move::empty());
        } else {
            returnVec.push(Move::get(moveIndex, PP, PPUp).unwrap());
        }
    }

    return returnVec;
}

    /// Function for retrieving a Pokemons Individual Values 
    /// Also known as [Determinant Values](https://m.bulbapedia.bulbagarden.net/wiki/Individual_values)
    /// 
    /// **NOTE**: This function does not have a bit manip test as the original implementation
    /// was faulty. 
    fn getPokemonIVsFromSave(save: &Vec<u8>,currAddr: &usize) -> [u16;5]{

        let atk = (save[currAddr+IV_OFF] >> 4) as u16;
        let def = ((save[currAddr+IV_OFF] << 4) >> 4) as u16;
        let spd = (save[currAddr+IV_OFF+1] >> 4) as u16;
        let spc = ((save[currAddr+IV_OFF+1] << 4) >> 4) as u16;

       
        // For the HPIVs we have to get the last bits of all four stats
        let mut hp = 0;
        hp |= (atk & 1) << 3 | (def & 1) << 2 | (spd & 1) << 1 | (spc & 1);

        let ivs: [u16;5] = [
                                hp,
                                atk,
                                def,
                                spd,
                                spc
                            ];
        
        return ivs;
    }

}

#[cfg(test)]
mod fileLoadingTests {
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

}

#[cfg(test)]
mod trainerTests {
    use super::*;

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
    fn setTrainerName_Incorrect() {
        // This is the save we'll be testing with
        let mut currSave = Save::new();

        // This is the trainer name we'll be setting.
        // The maximum length for a name in gen 1 is 7 chars.
        // Thus we will be testing with more than 7
        let newName = String::from("Professor Oak");

        // Now we change the name
        let nameChangeResult = currSave.setTrainerName(newName);

        // Once Gotten, we expect it to be Ok() and not Err()
        assert!(nameChangeResult.is_err());
        // And we expect the unwrapped version of the name to be true
        assert_eq!(nameChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Name \"Professor Oak\" is over 7 characters.");
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

}

#[cfg(test)]
mod basicPartyPokemonSetterTests {
    use super::*;
    use std::u16;

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

        // Length 10, boundary value
        let newNickname = String::from("Jimbosarus");

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
        assert_eq!(nicknameChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: Length of nickname \"Jimbosaurus Rex\" is over 10 characters.");
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
    fn setPartyPokemonLevel_IncorrectIndex() {
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

    #[test]
    fn setPartyPokemonOTID_Correct() {
        let mut testSave = Save::new();

        let newOTID = u16::MAX;

        let changeOTIDResult = testSave.setPartyPokemonOTID(0, newOTID);

        assert!(changeOTIDResult.is_ok());
        assert_eq!(changeOTIDResult.unwrap(), true);
    }

    #[test]
    fn setPartyPokemonOTID_IncorrectIndex() {
        let mut testSave = Save::new();

        let newOTID = u16::MAX;

        let changeOTIDResult = testSave.setPartyPokemonOTID(1, newOTID);

        assert!(changeOTIDResult.is_err());
        assert_eq!(changeOTIDResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1");
    }

    #[test]
    fn setPartyPokemonOTN_CorrectIndex() {
        let mut testSave = Save::new();

        let newOTN = String::from("Brock");

        let nicknameChangeResult = testSave.setPartyPokemonOTN(0, newOTN);

        assert!(nicknameChangeResult.is_ok());
        assert_eq!(nicknameChangeResult.unwrap(), true);
    }

    #[test]
    fn setPartyPokemonOTN_IncorrectIndex() {
        let mut testSave = Save::new();

        let newOTN = String::from("Brock");

        let nicknameChangeResult = testSave.setPartyPokemonOTN(1, newOTN);

        assert!(nicknameChangeResult.is_err());
        assert_eq!(nicknameChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1");
    }

    #[test]
    fn setPartyPokemonOTN_CorrectLength() {
        let mut testSave = Save::new();

        let newOTN = String::from("Brock");

        let nicknameChangeResult = testSave.setPartyPokemonOTN(0, newOTN);

        assert!(nicknameChangeResult.is_ok());
        assert_eq!(nicknameChangeResult.unwrap(), true);
    }

    #[test]
    fn setPartyPokemonOTN_IncorrectLength() {
        let mut testSave = Save::new();

        let newOTN = String::from("Professor Oak");

        let nicknameChangeResult = testSave.setPartyPokemonOTN(0, newOTN);

        assert!(nicknameChangeResult.is_err());
        assert_eq!(nicknameChangeResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: OTN \"Professor Oak\" is over 7 characters.");
    }

}

#[cfg(test)]
mod EVPartyPokemonSetterTests {
    use super::*;

    #[test]
    fn setPartyPokemonEV_HP_Correct() {
        let mut testSave = Save::new();

        let newHP: u16 = 65_535;

        let changeHPResult = testSave.setPartyPokemonEV_HP(0, newHP);

        assert!(changeHPResult.is_ok());
        assert_eq!(changeHPResult.unwrap(), true); 
    }

    #[test]
    fn setPartyPokemonEV_HP_IncorrectIndex() {
        let mut testSave = Save::new();

        let newHP: u16 = 65_535;

        let changeHPResult = testSave.setPartyPokemonEV_HP(1, newHP);

        assert!(changeHPResult.is_err());
        assert_eq!(changeHPResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1"); 
    }

    #[test]
    fn setPartyPokemonEV_ATK_Correct() {
        let mut testSave = Save::new();

        let newATK: u16 = 65_535;

        let changeATKResult = testSave.setPartyPokemonEV_ATK(0, newATK);

        assert!(changeATKResult.is_ok());
        assert_eq!(changeATKResult.unwrap(), true); 
    }

    #[test]
    fn setPartyPokemonEV_ATK_IncorrectIndex() {
        let mut testSave = Save::new();

        let newATK: u16 = 65_535;

        let changeATKResult = testSave.setPartyPokemonEV_ATK(1, newATK);

        assert!(changeATKResult.is_err());
        assert_eq!(changeATKResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1"); 
    }

    #[test]
    fn setPartyPokemonEV_DEF_Correct() {
        let mut testSave = Save::new();

        let newDEF: u16 = 65_535;

        let changeDEFResult = testSave.setPartyPokemonEV_HP(0, newDEF);

        assert!(changeDEFResult.is_ok());
        assert_eq!(changeDEFResult.unwrap(), true); 
    }

    #[test]
    fn setPartyPokemonEV_DEF_IncorrectIndex() {
        let mut testSave = Save::new();

        let newDEF: u16 = 65_535;

        let changeDEFResult = testSave.setPartyPokemonEV_DEF(1, newDEF);

        assert!(changeDEFResult.is_err());
        assert_eq!(changeDEFResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1"); 
    }

    #[test]
    fn setPartyPokemonEV_SPD_Correct() {
        let mut testSave = Save::new();

        let newSPD: u16 = 65_535;

        let changeSPDResult = testSave.setPartyPokemonEV_SPD(0, newSPD);

        assert!(changeSPDResult.is_ok());
        assert_eq!(changeSPDResult.unwrap(), true); 
    }

    #[test]
    fn setPartyPokemonEV_SPD_IncorrectIndex() {
        let mut testSave = Save::new();

        let newSPD: u16 = 65_535;

        let changeSPDResult = testSave.setPartyPokemonEV_SPD(1, newSPD);

        assert!(changeSPDResult.is_err());
        assert_eq!(changeSPDResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1"); 
    }

    #[test]
    fn setPartyPokemonEV_SPC_Correct() {
        let mut testSave = Save::new();

        let newSPC: u16 = 65_535;

        let changeSPCResult = testSave.setPartyPokemonEV_SPC(0, newSPC);

        assert!(changeSPCResult.is_ok());
        assert_eq!(changeSPCResult.unwrap(), true); 
    }

    #[test]
    fn setPartyPokemonEV_SPC_IncorrectIndex() {
        let mut testSave = Save::new();

        let newSPC: u16 = 65_535;

        let changeSPCResult = testSave.setPartyPokemonEV_SPC(1, newSPC);

        assert!(changeSPCResult.is_err());
        assert_eq!(changeSPCResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1"); 
    }

}

#[cfg(test)]
mod IVPartyPokemonSetterTests {
    use super::*;

    #[test]
    fn setPartyPokemonIV_HP_Correct() {
        let mut testSave = Save::new();

        let newHP:u16 = 15;

        let changeHPResult = testSave.setPartyPokemonIV_HP(0, newHP);

        assert!(changeHPResult.is_ok());
        assert_eq!(changeHPResult.unwrap(), true);
    }

    #[test]
    fn setPartyPokemonIV_HP_IncorrectIndex() {
        let mut testSave = Save::new();

        let newHP:u16 = 15;

        let changeHPResult = testSave.setPartyPokemonIV_HP(1, newHP);

        assert!(changeHPResult.is_err());
        assert_eq!(changeHPResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1");
    }

    #[test]
    fn setPartyPokemonIV_HP_IncorrectOver() {
        let mut testSave = Save::new();

        // Over the limit of 15
        let newHP:u16 = 16;

        let changeHPResult = testSave.setPartyPokemonIV_HP(0, newHP);

        assert!(changeHPResult.is_err());
        assert_eq!(changeHPResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: HP IV value is \"16\", which is over max value 15");
    }

    #[test]
    fn setPartyPokemonIV_ATK_Correct() {
        let mut testSave = Save::new();

        let newATK:u16 = 15;

        let changeATKResult = testSave.setPartyPokemonIV_ATK(0, newATK);

        assert!(changeATKResult.is_ok());
        assert_eq!(changeATKResult.unwrap(), true);
    }

    #[test]
    fn setPartyPokemonIV_ATK_IncorrectIndex() {
        let mut testSave = Save::new();

        let newATK:u16 = 15;

        let changeATKResult = testSave.setPartyPokemonIV_ATK(1, newATK);

        assert!(changeATKResult.is_err());
        assert_eq!(changeATKResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1");
    }

    #[test]
    fn setPartyPokemonIV_ATK_IncorrectOver() {
        let mut testSave = Save::new();

        // Over the limit of 15
        let newATK:u16 = 16;

        let changeATKResult = testSave.setPartyPokemonIV_ATK(0, newATK);

        assert!(changeATKResult.is_err());
        assert_eq!(changeATKResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: ATK IV value is \"16\", which is over max value 15");
    }

    #[test]
    fn setPartyPokemonIV_DEF_Correct() {
        let mut testSave = Save::new();

        let newDEF:u16 = 15;

        let changeDEFResult = testSave.setPartyPokemonIV_DEF(0, newDEF);

        assert!(changeDEFResult.is_ok());
        assert_eq!(changeDEFResult.unwrap(), true);
    }

    #[test]
    fn setPartyPokemonIV_DEF_IncorrectIndex() {
        let mut testSave = Save::new();

        let newDEF:u16 = 15;

        let changeDEFResult = testSave.setPartyPokemonIV_DEF(1, newDEF);

        assert!(changeDEFResult.is_err());
        assert_eq!(changeDEFResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1");
    }

    #[test]
    fn setPartyPokemonIV_DEF_IncorrectOver() {
        let mut testSave = Save::new();

        // Over the limit of 15
        let newDEF:u16 = 16;

        let changeDEFResult = testSave.setPartyPokemonIV_DEF(0, newDEF);

        assert!(changeDEFResult.is_err());
        assert_eq!(changeDEFResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: DEF IV value is \"16\", which is over max value 15");
    }

    #[test]
    fn setPartyPokemonIV_SPD_Correct() {
        let mut testSave = Save::new();

        let newSPD:u16 = 15;

        let changeSPDResult = testSave.setPartyPokemonIV_SPD(0, newSPD);

        assert!(changeSPDResult.is_ok());
        assert_eq!(changeSPDResult.unwrap(), true);
    }

    #[test]
    fn setPartyPokemonIV_SPD_IncorrectIndex() {
        let mut testSave = Save::new();

        let newSPD:u16 = 15;

        let changeSPDResult = testSave.setPartyPokemonIV_SPD(1, newSPD);

        assert!(changeSPDResult.is_err());
        assert_eq!(changeSPDResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1");
    }

    #[test]
    fn setPartyPokemonIV_SPD_IncorrectOver() {
        let mut testSave = Save::new();

        // Over the limit of 15
        let newSPD:u16 = 16;

        let changeSPDResult = testSave.setPartyPokemonIV_SPD(0, newSPD);

        assert!(changeSPDResult.is_err());
        assert_eq!(changeSPDResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: SPD IV value is \"16\", which is over max value 15");
    }

    #[test]
    fn setPartyPokemonIV_SPC_Correct() {
        let mut testSave = Save::new();

        let newSPC:u16 = 15;

        let changeSPCResult = testSave.setPartyPokemonIV_SPC(0, newSPC);

        assert!(changeSPCResult.is_ok());
        assert_eq!(changeSPCResult.unwrap(), true);
    }

    #[test]
    fn setPartyPokemonIV_SPC_IncorrectIndex() {
        let mut testSave = Save::new();

        let newSPC:u16 = 15;

        let changeSPCResult = testSave.setPartyPokemonIV_SPC(1, newSPC);

        assert!(changeSPCResult.is_err());
        assert_eq!(changeSPCResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: There is no Pokemon in party slot 1");
    }

    #[test]
    fn setPartyPokemonIV_SPC_IncorrectOver() {
        let mut testSave = Save::new();

        // Over the limit of 15
        let newSPC:u16 = 16;

        let changeSPCResult = testSave.setPartyPokemonIV_SPC(0, newSPC);

        assert!(changeSPCResult.is_err());
        assert_eq!(changeSPCResult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: SPC IV value is \"16\", which is over max value 15");
    }

}

#[cfg(test)]
/// These tests are specifically for sanity checking bit manipulation
/// as opposed to strings with radix conversions.
mod fileLoadingBitManipulationTests {
    use super::*;

    #[test]
    fn getTrainerIDFromSave_BitManipTest() {

        let testSaveList = ["./test/POKEMON YELLOW.sav", "./test/POKEMON YELLOW 2.sav", "./test/POKEMON BLUE.sav", "./test/Pokeblue.sav", "./test/POKEpi.sav"];

        for save in testSaveList {
            // Read a test save file
            let testSave = fs::read(save).unwrap();

            // Get the ID with bit manipulation
            let readID = Save::getTrainerIDFromSave(&testSave);
            // Get the expected ID with the simpled String-radix method
            let expectedlID = u16::from_str_radix(&format!("{:02X}{:02X}",testSave[ID_ADDR],testSave[ID_ADDR+1]), 16).unwrap();

            assert_eq!(readID, expectedlID);
        }
    }

    #[test]
    fn getPokemonOTIDFromSave_BitManipTest() {

        let testSaveList = ["./test/POKEMON YELLOW.sav", "./test/POKEMON YELLOW 2.sav", "./test/POKEMON BLUE.sav", "./test/Pokeblue.sav", "./test/POKEpi.sav"];

        for save in testSaveList {
            // Read a test save file
            let testSave = fs::read(save).unwrap();

            let testPkmnAddress: usize = PARTY_ADDR + 0x8 + 0x2C;

            // Get the ID with bit manipulation
            let readID = Save::getPokemonOTIDFromSave(&testSave, &testPkmnAddress);
            // Get the expected ID with the simpled String-radix method
            let expectedlID = u16::from_str_radix(&format!("{:02X}{:02X}", testSave[testPkmnAddress+OT_OFF],testSave[testPkmnAddress+OT_OFF+1]),16).unwrap();

            assert_eq!(readID, expectedlID);
        }
    }

    #[test]
    fn getPokemonHPFromSave_BitManipTest() {

        let testSaveList = ["./test/POKEMON YELLOW.sav", "./test/POKEMON YELLOW 2.sav", "./test/POKEMON BLUE.sav", "./test/Pokeblue.sav", "./test/POKEpi.sav"];

        for save in testSaveList {
            // Read a test save file
            let testSave = fs::read(save).unwrap();

            let testPkmnAddress: usize = PARTY_ADDR + 0x8 + 0x2C;

            // Get the HP with bit manipulation
            let readHP = Save::getPokemonHPFromSave(&testSave, &testPkmnAddress);
            // Get the expected HP with the simpled String-radix method
            let expectedHP = i16::from_str_radix(&format!("{:02X}{:02X}",testSave[testPkmnAddress+HP_OFF],testSave[testPkmnAddress+HP_OFF+1]), 16).unwrap();

            assert_eq!(readHP, expectedHP);
        }
    }

    #[test]
    fn getPokemonStatsFromSave_BitManipTest() {

        let testSaveList = ["./test/POKEMON YELLOW.sav", "./test/POKEMON YELLOW 2.sav", "./test/POKEMON BLUE.sav", "./test/Pokeblue.sav", "./test/POKEpi.sav"];

        for save in testSaveList {
            // Read a test save file
            let testSave = fs::read(save).unwrap();

            let testPkmnAddress: usize = PARTY_ADDR + 0x8 + 0x2C;

            // Get the stats with bit manipulation
            let readStats = Save::getPokemonStatsFromSave(&testSave, &testPkmnAddress);

            // Get the expected stats with the simpled String-radix method
            let mut expectedStats: [u16;5] = [12345; 5];
                for stat in 0..5 {
                    let currAddr = testPkmnAddress+STAT_OFF+(stat*2);
                    expectedStats[stat] = u16::from_str_radix(
                                                    &format!("{:02X}{:02X}",testSave[currAddr],testSave[currAddr+1]),
                                                    16
                                                ).unwrap();
                }

            
            assert_eq!(readStats, expectedStats);
        }
    }

    #[test]
    fn getPokemonEVsFromSave_BitManipTest() {

        let testSaveList = ["./test/POKEMON YELLOW.sav", "./test/POKEMON YELLOW 2.sav", "./test/POKEMON BLUE.sav", "./test/Pokeblue.sav", "./test/POKEpi.sav"];

        for save in testSaveList {
            // Read a test save file
            let testSave = fs::read(save).unwrap();

            let testPkmnAddress: usize = PARTY_ADDR + 0x8 + 0x2C;

            // Get the stats with bit manipulation
            let readEVs = Save::getPokemonEVsFromSave(&testSave, &testPkmnAddress);

            // Get the expected stats with the simpled String-radix method
            let mut expectedEVs: [u16;5] = [12345; 5];
                for ev in 0..5 {
                    let currAddr = testPkmnAddress+EV_OFF+(ev*2);
                    expectedEVs[ev] = u16::from_str_radix(
                                                    &format!("{:02X}{:02X}",testSave[currAddr],testSave[currAddr+1]),
                                                    16
                                                ).unwrap();
                }
            
            assert_eq!(readEVs, expectedEVs);
        }
    }

    #[test]
    fn getPokemonMovesFromSave_BitManipTest() {

        let testSaveList = ["./test/POKEMON YELLOW.sav", "./test/POKEMON YELLOW 2.sav", "./test/POKEMON BLUE.sav", "./test/Pokeblue.sav", "./test/POKEpi.sav"];

        for save in testSaveList {
             // Read a test save file
            let testSave = fs::read(save).unwrap();

            let testPkmnAddress: usize = PARTY_ADDR + 0x8 + 0x2C;

            // First, let's get the moves the old string way
            let mut expectedMoves: Vec<Move> = Vec::new();
            let moveAddr = testPkmnAddress + MOVE_OFF;
        
            for moves in 0..4 {
                let moveIndex = testSave[moveAddr+moves] as u16;
                let ppStr = format!("{:08b}",testSave[testPkmnAddress+PP_OFF+moves]);
                let (ppUp,pp) = ppStr.split_at(2);
                let currPP = u16::from_str_radix(pp, 2).unwrap();
                let currPPUp = u8::from_str_radix(ppUp, 2).unwrap();
                if moveIndex == 0 {
                    expectedMoves.push(Move::empty());
                } else {
                    expectedMoves.push(Move::get(moveIndex, currPP, currPPUp).unwrap());
                }
            }

            // Now we do it the new way
            let actualMoves = Save::getPokemonMovesFromSave(&testSave, &testPkmnAddress);

            // Now from these moves, let's create an Array for the PP, and PPUp
            let mut expectedPPArr: [&u16; 4] = [&0; 4];
            let mut expectedPPUpArr: [&u8; 4] = [&0; 4];

            let mut actualPPArr: [&u16; 4] = [&0; 4];
            let mut actualPPUpArr: [&u8; 4] = [&0; 4];

            for index in 0..4 {
                expectedPPArr[index] = expectedMoves[index].getPP();
                expectedPPUpArr[index] = expectedMoves[index].getPPUp();
                actualPPArr[index] = actualMoves[index].getPP();
                actualPPUpArr[index] = actualMoves[index].getPPUp();
            }

            // And finally, we assert that all these values are the same
            assert_eq!(actualPPArr,expectedPPArr);
            assert_eq!(actualPPUpArr,expectedPPUpArr);

        }
       
        

    }

}