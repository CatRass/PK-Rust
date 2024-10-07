/// Module for organising all data related to Pokemon.
/// 
/// This includes data such as:
/// - Types
/// - Moves
/// - Various Stats
/// - Species

use std::fs;

use PkmnType::Type;
use PkmnMove::Move;
use super::addresses::{MOVE_OFF,PP_OFF};

pub mod PkmnType;
pub mod PkmnMove;

#[derive(Debug)]
enum StatusCondition {

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

/// Function for retrieving data about Pokemons moves.
pub fn getPokemonMoves(save: &Vec<u8>, currAddr: &usize) -> Vec<Move>{
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
