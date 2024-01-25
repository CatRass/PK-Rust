// Pokemon Module
use std::fs;
use std::process;

const MONEY_ADDR: usize     = 0x25F3;
const PLAYER_ID: usize      = 0x2605;

#[allow(non_snake_case)]
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
    pub trainer: String,
    /// The amount of money the player holds.
    /// 
    /// [Source](https://bulbapedia.bulbagarden.net/wiki/Save_data_structure_(Generation_I)#bank1_main_money)
    pub money: i16,
    pub id: i32,
    // party: (CreatureData::Pokemon, CreatureData::Pokemon)
}

#[allow(non_snake_case)]
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

        
        return Save{trainer: "NULL".to_string(), money, id}

    }

    /// Print the save file data to terminal
    pub fn print(self: Self) {
        println!("\n=== Save Info ===");
        println!("Name: {}\nPlayer ID: {}\nMoney: {}",self.trainer, self.id, self.money);
        println!("=================\n");
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

/// **TODO**: Optimise this!
fn hex_to_dec(mut hex_num: i16) -> i32{
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