#![allow(non_snake_case)]

// Pokemon Module
use std::fs;
use std::process;

const MONEY_ADDR: usize     = 0x25F3;
const PLAYER_ID: usize      = 0x2605;
const NAME_ADDR: usize      = 0x2598;

#[allow(dead_code)]
pub mod CreatureData {
    pub(crate) struct Pokemon {
        nickname: String,
        typing: (Type,Type),
        level: i8,
        moves: (Move,Move,Move,Move),
    }

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

    // struct Move {
    //     name: String,
    //     typing: Type,
    // }

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
}

pub struct Save {
    trainer: Name,
    /// The amount of money the player holds.
    /// 
    /// [Source](https://bulbapedia.bulbagarden.net/wiki/Save_data_structure_(Generation_I)#bank1_main_money)
    money: i16,
    id: i32,
    // party: (CreatureData::Pokemon, CreatureData::Pokemon)
}
struct Name {
    encoded: [i16; 11],
    text: String
}

impl Save {
    
    pub fn load(file: &str) -> Save{

        let test = match fs::read(file) {
            Ok(result)                => result,
            Err(error)                  => match error.kind() {
                std::io::ErrorKind::NotFound   => {eprintln!("File does note exist"); process::exit(1);}
                _                              => {eprintln!("Error: {}",error.kind()); process::exit(1);}
            }
        };

        // Conversion from Hex-Coded Decimal
        let money = Self::getMoney(&test);
        let id = Self::getID(&test);

        let trainerName = Self::getName(&test);
        let trainer = Name{encoded: trainerName, text: textDecode(&trainerName)};

        
        return Save{trainer, money, id}

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

    fn getMoney(save: &Vec<u8>) -> i16{
        return format!("{:X}{:X}{:X}",save[MONEY_ADDR],save[MONEY_ADDR+1],save[MONEY_ADDR+2])
        .parse::<i16>()
        .unwrap();
    }

    fn getID(save: &Vec<u8>) -> i32 {
        let hexId =
            format!("{:X}",save[PLAYER_ID]).parse::<i16>().unwrap()*100 +
            format!("{:X}",save[PLAYER_ID+1]).parse::<i16>().unwrap()*1
        ;

        return hex_to_dec(hexId);
    }
}

/// A function for converting hexidecimal numbers to decimal
/// 
/// Note that the function currently only works for numeric hex numbers, meaning no A-F digits
/// 
/// **TODO**: Optimise this!
fn hex_to_dec(mut hex_num: i16) -> i32 {
    let mut dec_num: i32 = 0;
    let mut multiplier: i32 = 1;

    while hex_num > 0 {
        let curr_place = hex_num as i32 %10 * multiplier;
        dec_num = dec_num + curr_place as i32;
        multiplier = multiplier * 16;
        hex_num = hex_num/10;
    }

    return dec_num;
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