#![allow(non_snake_case)]

use std::fs;
use std::path::PathBuf;
use std::process;
use crate::CreatureData::{Pokemon, Move};

// General Starting Addresses
const MONEY_ADDR:       usize   = 0x25F3;
const ID_ADDR:          usize   = 0x2605;
const NAME_ADDR:        usize   = 0x2598;
const PARTY_ADDR:       usize   = 0x2F2C;
const PC_ADDR:          usize   = 0x4000;

// PC Offsets

/// Pokemon Species Index
const PC_PKMN_OFF:      usize   = 0x16;
/// Original Trainer Name
const PC_TRAINER_OFF:   usize   = 0x2AA;
/// Pokemon Nickname Offset
const PC_NICK_OFF:      usize   = 0x386;

// Pokemon Data Offsets
const NICK_OFF:         usize   = 0x152;
const HP_OFF:           usize   = 0x01;
const MOVE_OFF:         usize   = 0x08;
const PP_OFF:           usize   = 0x1D;
const OT_OFF:           usize   = 0x0C;
const OTN_OFF:          usize   = 0x110;
const EV_OFF:           usize   = 0x11;
const STAT_OFF:         usize   = 0x22;
const IV_OFF:           usize   = 0x1B;

/// Module for organising all data related to Pokemon.
/// 
/// This includes data such as:
/// - Types
/// - Moves
/// - Various Stats
/// - Species
#[allow(dead_code)]
pub mod CreatureData {
    use std::fs;

    #[derive(Debug)]
    enum StatusCondition {

    }

    #[derive(Debug)]
    enum Type {
        Normal      = 0,
        Fire        = 1,
        Fighting    = 2,
        Water       = 3,
        Flying      = 4,
        Grass       = 5,
        Poison      = 6,
        Electric    = 7,
        Ground      = 8,
        Psychic     = 9,
        Rock        = 10,
        Ice         = 11,
        Bug         = 12,
        Dragon      = 13,
        Ghost       = 14,
        Dark        = 15,
        Steel       = 16,
        Fairy       = 17,
    
        // For single type pokemon
        Null
    }
    impl Type {
        pub(crate) fn get(index: &i16) -> Type{
            let returnType = match index {
                0   => Type::Normal   ,
                1   => Type::Fire     ,
                2   => Type::Fighting ,
                3   => Type::Water    ,
                4   => Type::Flying   ,
                5   => Type::Grass    ,
                6   => Type::Poison   ,
                7   => Type::Electric ,
                8   => Type::Ground   ,
                9   => Type::Psychic  ,
                10  => Type::Rock     ,
                11  => Type::Ice      ,
                12  => Type::Bug      ,
                13  => Type::Dragon   ,
                14  => Type::Ghost    ,
                15  => Type::Dark     ,
                16  => Type::Steel    ,
                17  => Type::Fairy    ,
                18  => Type::Null     ,
                _   => Type::Null
            };

            returnType
        }
    }

    #[derive(Debug)]

    pub(crate) struct Move {
        index: u16,
        typing: Type,
        name: String,
        pp: u16,
        ppup: u8
    }
    impl Move {
        /// Constructor for a Move, given an input move index
        pub(crate) fn get(index: u16, pp: u16, ppup: u8) -> Move {
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
        pub(crate) fn empty() -> Move {
            return Move{index:0, typing: Type::Null, name: String::from("Null"), pp:0, ppup:0}
        }
        /// Returns the info on a Pokemons moves for printing
        fn to_string(&self) -> String {
            return format!("{} PP: {} PP Up: {}", self.name, self.pp, self.ppup);
        }

    }

    #[derive(Debug)]

    struct Species {
        index: i16,
        pokedex: i16,
        name: String,
        typing: [Type;2],
    }
    impl Species {
        fn parse(index: i16) -> Species {
            let speciesFile = fs::read_to_string("./data/species.pkmn").unwrap();
            let mut parsedSpecies: &str = "No Pokemon found";

            let hexIndex = format!("0x{:02X?}",index);
            // println!("{}",hexIndex);
            
            for line in speciesFile.lines() {
                if line.contains(&hexIndex) {
                    parsedSpecies = line;
                    break;
                }
            }

            let info: Vec<&str> = parsedSpecies.split(" ").collect();
            let pokedex = info[0].parse::<i16>().unwrap();
            let name = info[2].to_string();

            // PLEASE fix this later, I don't even want to explain what horribleness I wrote here
            let types: Vec<&str> = info[3].trim_matches('{').trim_matches('}').split(',').collect();
            let typing: [Type;2] = [
                                    Type::get(&(types[0].parse::<i16>().unwrap())), 
                                    Type::get(&(types[1].parse::<i16>().unwrap()))
                                   ];

            return Species{index,pokedex,name,typing};
        }
    }

    #[derive(Debug)]

    struct EVs {
        hp: u16,
        atk: u16,
        def: u16,
        spd: u16,
        spc: u16
    }
    impl EVs {
        /// Returns a string to display all EVs
        fn to_string(&self) -> String {
            return format!("\tHP EV: {}\n\tATK EV: {}\n\tDEF EV:{}\n\tSPD EV: {}\n\tSPCL EV: {}\n",
                            self.hp,
                            self.atk,
                            self.def,
                            self.spd,
                            self.spc
                        );
        }
    }

    #[derive(Debug)]

    struct Stats {
        hp: u16,
        atk: u16,
        def: u16,
        spd: u16,
        spc: u16
    }
    impl Stats {
        /// Returns a string to display all stats
        fn to_string(&self) -> String {
            return format!("\tHP: {}\n\tATK: {}\n\tDEF:{}\n\tSPD: {}\n\tSPCL: {}\n",
                            self.hp,
                            self.atk,
                            self.def,
                            self.spd,
                            self.spc
                        );
        }
    }

    #[derive(Debug)]

    struct IVs {
        hp:  u16,
        atk: u16,
        def: u16,
        spd: u16,
        spc: u16
    }
    impl IVs {
        /// Returns a string to display all stats
        fn to_string(&self) -> String {
            return format!("\tATK IV: {}\n\tDEF IV:{}\n\tSPD IV: {}\n\tSPCL IV: {}\n",
                            self.atk,
                            self.def,
                            self.spd,
                            self.spc
                        );
        }
    }

    #[derive(Debug)]

    pub(crate) struct Pokemon {
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
        pub(crate) fn get(index: i16, level:i8, nickname: String, moves: Vec<Move>, ot: u16, otn: String, hp: i16, evArr: [u16;5], ivArr: [u16;5], statArr: [u16;5]) -> Pokemon {
            let species = Species::parse(index);

            let evs = EVs{hp: evArr[0], atk: evArr[1], def: evArr[2], spd: evArr[3], spc: evArr[4]};
            let ivs = IVs{atk: ivArr[0], def: ivArr[1], spd: ivArr[2], spc: ivArr[3], hp: ivArr[4]};
            let stats = Stats{hp: statArr[0], atk: statArr[1], def: statArr[2], spd: statArr[3], spc: statArr[4]};
            
            return Pokemon{nickname, species, level, moves, ot, otn, hp, evs, ivs, stats};
        }

        /// Returns a string with all of the Pokemon's details, such as:
        /// 
        /// - Species
        /// - Nickname
        /// - Level
        /// - Current HP
        /// - EVs
        /// - IVs
        /// - Stats
        pub(crate) fn getDetails(&self) -> String{
            let basicDetails= format!("{:12} {:12} LVL:{} Current HP: {}\n",
                                            self.species.name, 
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
    }

}

#[derive(Debug)]
pub struct Save {
    trainer: String,
    money: u32,
    id: i32,
    party: Vec<Pokemon>,
    // Each save file has 12 boxes, which hold 20 pokemon each
    // Pokemon Box info here: https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_Storage_System
    pc: Vec<Vec<Pokemon>>
}

impl Save {

    pub fn new() -> Save {
        return Save{    trainer: String::from("Null"),
                        money: 0,
                        id: 0,
                        party: Vec::new(),
                        pc: Vec::new()
                    }
    } 

    pub fn load(file: &PathBuf) -> Save{

        let save = match fs::read(file) {
            Ok(result)                => result,
            Err(error)                  => match error.kind() {
                std::io::ErrorKind::NotFound   => {eprintln!("Save: {} does not exist",file.to_str().unwrap()); process::exit(1);}
                _                              => {eprintln!("Error: {}",error.kind()); process::exit(1);}
            }
        };

        let pc = Self::getPCBoxes(&save);

        let money = Self::getMoney(&save);
        let id = Self::getTrainerID(&save);
        let party:  Vec<Pokemon> = Self::getParty(&save);
        let trainer = textDecode(&Self::getName(&save));

        return Save{trainer, money, id, party, pc}

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

    /// Retrieves the name from the save file
    /// 
    /// Since most Pokemon games use character encoding, we have to decode it.
    /// 
    /// [Gen 1](https://bulbapedia.bulbagarden.net/wiki/Character_encoding_(Generation_I))
    fn getName(save: &Vec<u8>) -> [i16;11] {
        let mut name: [i16; 11] = [0; 11];
        for num in 0..11 {
            name[num] = format!("{}",save[NAME_ADDR+num]).parse::<i16>().unwrap();
        }
        return name;
    }

    /// Retrieves the amount of money the player has
    fn getMoney(save: &Vec<u8>) -> u32{
        return format!("{:02X}{:02X}{:02X}",save[MONEY_ADDR],save[MONEY_ADDR+1],save[MONEY_ADDR+2])
        .parse::<u32>()
        .unwrap();
    }

    /// Retrieves the trainer ID
    fn getTrainerID(save: &Vec<u8>) -> i32 {
        let stringID = format!("{:02X}{:02X}",save[ID_ADDR],save[ID_ADDR+1]);
        return i32::from_str_radix(&stringID, 16).unwrap();
    }

    /// Retrieves the players party of Pokemon
    fn getParty(save: &Vec<u8>) -> Vec<Pokemon> {
        let mut party:  Vec<Pokemon> = Vec::new();

        for creature in 0..save[PARTY_ADDR] as usize {
            let pkmnAddress: usize = PARTY_ADDR + 0x8 + (creature * 0x2C);
            let nickAddress: usize = PARTY_ADDR + NICK_OFF + (creature * 0xB);

            // Get current HP
            let hp = Self::getPokemonHP(&save,&pkmnAddress);
            // Nickname Obtaining code
            let nickname = Self::getPokemonNick(&save, &nickAddress);
            // Moves Obtaining code
            let moves = Self::getPokemonMoves(&save,&pkmnAddress);
            // EV Obtaining code
            let evs: [u16;5] = Self::getPokemonEVs(&save,&pkmnAddress);
            // Stat Obtaining Code
            let stats: [u16;5] = Self::getPokemonStats(&save,&pkmnAddress);
            // IV Obtaining Code
            let ivs: [u16;5] = Self::getPokemonIVs(&save,&pkmnAddress);
            // Original Trainer Obtaining Code
            let ot = Self::getPokemonOTID(&save,&pkmnAddress);
            let otn = Self::getPokemonOTName(&save, &(pkmnAddress+OTN_OFF));

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
    fn getPCBoxes(save: &Vec<u8>) -> Vec<Vec<Pokemon>>{
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
                let hp = Self::getPokemonHP(&save, &pkmnAddress);
                let ot = Self::getPokemonOTID(&save, &pkmnAddress);
                let moves = Self::getPokemonMoves(&save,&pkmnAddress);
                let nickname = Self::getPokemonNick(&save, &nickAddress);
                let evs: [u16;5] = Self::getPokemonEVs(&save,&pkmnAddress);
                let ivs: [u16;5] = Self::getPokemonIVs(&save,&pkmnAddress);
                let level: i8 = save[pkmnAddress+0x03] as i8;
                let otn = Self::getPokemonOTName(&save, &(currAddr+PC_TRAINER_OFF+(creature*0xB)));
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
    fn getPokemonOTID(save: &Vec<u8>, currAddr: &usize) -> u16{
        return u16::from_str_radix( 
            &format!("{:02X}{:02X}", save[currAddr+OT_OFF],save[currAddr+OT_OFF+1]),
            16
        ).unwrap();
    }

    /// Function for retrieving a Pokemons Original Trainers Name
    /// 
    /// **Note**: There is a current bug where extra "garbage data" is added to OT Names.
    /// This most likely to do with the function textDecode note taking into account control characters.
    fn getPokemonOTName(save: &Vec<u8>, currAddr: &usize) -> String {
        let mut encodedName: [i16;11] = [0;11];

        for char in 0..11 {
            encodedName[char] = format!("{}",save[currAddr+char]).parse::<i16>().unwrap();
        }

        return textDecode(&encodedName);
    }

    /// Function for retrieving the Pokemons current Health Points
    fn getPokemonHP(save: &Vec<u8>, currAddr: &usize) -> i16{
        return i16::from_str_radix(
            &format!("{:02X}{:02X}",save[currAddr+HP_OFF],save[currAddr+HP_OFF+1]), 
            16
            ).unwrap();
    }

    /// Function for retrieving data about Pokemons moves.
    fn getPokemonMoves(save: &Vec<u8>, currAddr: &usize) -> Vec<Move>{
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
                returnVec.push(Move::get(moveIndex, currPP, currPPUp));
            }
        }

        return returnVec;
    }

    /// Function for retrieving a Pokemons Nickname.
    /// 
    /// **Note**: This function will automatically decode it into a String
    fn getPokemonNick(save: &Vec<u8>,currAddr: &usize) -> String {
        let mut encodedNick: [i16; 11]= [0; 11];
            for num in 0..11 {
                encodedNick[num] = format!("{}",save[currAddr+num]).parse::<i16>().unwrap();
            }
        return textDecode(&encodedNick);
    }

    /// Function for retrieving a Pokemons base stats
    fn getPokemonStats(save: &Vec<u8>,currAddr: &usize) -> [u16;5] {
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
    fn getPokemonEVs(save: &Vec<u8>,currAddr: &usize) -> [u16;5] {
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

    /// Function for retrieving a Pokemons Individual Values 
    /// (Also known as Determinant Values)
    /// 
    /// **TODO**: Figure out a better way to split into fours
    fn getPokemonIVs(save: &Vec<u8>,currAddr: &usize) -> [u16;5]{
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

/// Decodes text, as text in most games uses character encoding
/// **TODO**: Optimise this too. Yanderedev levels of code
fn textDecode(encoded: &[i16; 11]) -> String{
    let mut name: Vec<u8> = Vec::new();

    for char in encoded {
        name.push(match char {
            0x80 => b'A',
            0x81 => b'B',
            0x82 => b'C',
            0x83 => b'D',
            0x84 => b'E',
            0x85 => b'F',
            0x86 => b'G',
            0x87 => b'H',
            0x88 => b'I',
            0x89 => b'J',
            0x8A => b'K',
            0x8B => b'L',
            0x8C => b'M',
            0x8D => b'N',
            0x8E => b'O',
            0x8F => b'P',
            0x90 => b'Q',
            0x91 => b'R',
            0x92 => b'S',
            0x93 => b'T',
            0x94 => b'U',
            0x95 => b'V',
            0x96 => b'W',
            0x97 => b'X',
            0x98 => b'Y',
            0x99 => b'Z',
            0x9A => b'(',
            0x9B => b')',
            0x9C => b':',
            0x9D => b';',
            0x9E => b'[',
            0x9F => b']',
            0xA0 => b'a',
            0xA1 => b'b',
            0xA2 => b'c',
            0xA3 => b'd',
            0xA4 => b'e',
            0xA5 => b'f',
            0xA6 => b'g',
            0xA7 => b'h',
            0xA8 => b'i',
            0xA9 => b'j',
            0xAA => b'k',
            0xAB => b'l',
            0xAC => b'm',
            0xAD => b'n',
            0xAE => b'o',
            0xAF => b'p',
            0xB0 => b'q',
            0xB1 => b'r',
            0xB2 => b's',
            0xB3 => b't',
            0xB4 => b'u',
            0xB5 => b'v',
            0xB6 => b'w',
            0xB7 => b'x',
            0xB8 => b'y',
            0xB9 => b'z',
            0xBA => b'\xE9',
            _   => b' ' 
        }); 
    }

    return String::from_utf8(name).unwrap();
}

// /// Encodes text into the character encoding used by Gen 1
// /// **TODO**: Implement this!
// fn textEncode(decoded: &String) -> [i16; 11]{
//     todo!("Finish this, preferrably optimised");
// }