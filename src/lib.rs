#![allow(non_snake_case)]

// Pokemon Module
use std::fs;
use std::process;

use crate::CreatureData::Pokemon;

const MONEY_ADDR:   usize     = 0x25F3;
const ID_ADDR:      usize     = 0x2605;
const NAME_ADDR:    usize     = 0x2598;
const PARTY_ADDR:   usize     = 0x2F2C;

#[allow(dead_code)]
pub mod CreatureData {
    use std::fs;

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

    enum Move {
        /// A move will consist of its:
        /// - Move ID (1 byte long)
        /// - Name (String)
        /// - Type
        Move(i8,String,Type),

        /// A null move is used in place of a move slot 
        /// that has not been allocated a move
        Null
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
            let speciesFile = fs::read_to_string("./src/species.pkmn").unwrap();
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
    pub(crate) struct Pokemon {
        nickname: String,
        species: Species,
        level: i8,
        // moves: [Move; 4],
    }

    impl Pokemon {
        pub(crate) fn get(index: i16, level:i8, nickname: String) -> Pokemon {
            let species = Species::parse(index);

            return Pokemon{nickname,species,level};
        }
    }

}

struct Name {
    encoded: [i16; 11],
    text: String
}

pub struct Save {
    trainer: Name,
    /// The amount of money the player holds.
    /// 
    /// [Source](https://bulbapedia.bulbagarden.net/wiki/Save_data_structure_(Generation_I)#bank1_main_money)
    money: u32,
    id: i32,
    party: Vec<Pokemon>
}

impl Save {
    
    pub fn load(file: &str) -> Save{

        let save = match fs::read(file) {
            Ok(result)                => result,
            Err(error)                  => match error.kind() {
                std::io::ErrorKind::NotFound   => {eprintln!("File does not exist"); process::exit(1);}
                _                              => {eprintln!("Error: {}",error.kind()); process::exit(1);}
            }
        };

        // Conversion from Hex-Coded Decimal
        let money = Self::getMoney(&save);
        let id = Self::getID(&save);

        let trainerName = Self::getName(&save);
        let trainer = Name{encoded: trainerName, text: textDecode(&trainerName)};

        let mut party:  Vec<Pokemon> = Vec::new();

        for creature in 0..save[PARTY_ADDR] as usize {
            let pkmnAddress = PARTY_ADDR + 0x8 + (creature * 0x2C);
            let nickAddress: usize = PARTY_ADDR + 0x152 + (creature * 0xB);

            let mut encodedNick: [i16; 11]= [0; 11];
            for num in 0..11 {
                encodedNick[num] = format!("{}",save[nickAddress+num]).parse::<i16>().unwrap();
            }

            let nickname = textDecode(&encodedNick);

            party.push(Pokemon::get(save[pkmnAddress] as i16, save[pkmnAddress+0x21] as i8, nickname));
            println!("Current Pokemon: {:?}", party[creature]);
        }

        return Save{trainer, money, id, party}

    }

    /// Print the save file data to terminal
    pub fn print(self: Self) {
        println!("\n=== Save Info ===");
        println!("Name: {}\nPlayer ID: {}\nMoney: {}",self.trainer.text, self.id, self.money);
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

    fn getMoney(save: &Vec<u8>) -> u32{
        return format!("{:X}{:X}{:X}",save[MONEY_ADDR],save[MONEY_ADDR+1],save[MONEY_ADDR+2])
        .parse::<u32>()
        .unwrap();
    }

    fn getID(save: &Vec<u8>) -> i32 {
        let stringID = format!("{:X?}{:X?}",save[ID_ADDR],save[ID_ADDR+1]);
        return i32::from_str_radix(&stringID, 16).unwrap();
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