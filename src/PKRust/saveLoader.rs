use std::fs;
use std::path::PathBuf;

use super::creatureData::pokemonMove::Move;
use super::creatureData::pokemon::*;
use super::addresses::*;
use super::utils::{textDecode, integrityCheck, formatError};


#[derive(Debug)]
#[allow(dead_code)]
pub struct Save {
    trainer: String,
    money: u32,
    id: i32,
    party: Vec<Pokemon>,
    // Each save file has 12 boxes, which hold 20 pokemon each
    // Pokemon Box info here: https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_Storage_System
    pc: Vec<Vec<Pokemon>>
}

#[allow(dead_code)]
impl Save {

    pub fn new() -> Save {
        return Save{    trainer: String::from("Null"),
                        money: 0,
                        id: 0,
                        party: Vec::new(),
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

    /// Getter for Trainer Name in Save
    pub fn getTrainerName(&self) -> &String {
        return &self.trainer;
    }    

    /// Getter for Money in Save
    pub fn getMoney(&self) -> &u32 {
        return &self.money;
    }

    /// Getter for Trainer ID in Save
    pub fn getTrainerID(&self) -> &i32 {
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
    fn getTrainerIDFromSave(save: &Vec<u8>) -> i32 {
        let stringID = format!("{:02X}{:02X}",save[ID_ADDR],save[ID_ADDR+1]);
        return i32::from_str_radix(&stringID, 16).unwrap();
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

}