use std::fs;
use super::Type;

#[derive(Debug)]
pub struct Species {
    index: i16,
    pokedex: i16,
    // TODO: Get a Getter
    pub name: String,
    typing: [Type;2],
}
impl Species {
    pub fn parse(index: i16) -> Species {
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
