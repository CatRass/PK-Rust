use std::fs;
use super::Type;

#[derive(Debug)]
/**
 * The object that stores a the Pokemon Species
 * index: The index given to a pokemon sprite (0x01 being Rhydon)
 * pokedex: The number of the pokemon in the pokedex
 * name: The string name of the species
 * typing: an array holding the two types of a species
 */
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
        println!("{}",hexIndex);
        
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

    pub fn getIndex(&self) -> &i16 {
        return &self.index;
    }

    pub fn getPokedex(&self) -> &i16 {
        return &self.pokedex;
    }

    pub fn getName(&self) -> &String {
        return &self.name;
    }

    pub fn getTyping(&self) -> &[Type;2] {
        return &self.typing;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_testCorrectPokemon() {
        // Id of the pokemon
        let id = 0x99;
        let parsedSpecies: Species = Species::parse(id);

        let correctSpecies:Species = Species {
            index: 0x99, 
            pokedex: 1, 
            name: "Bulbasaur".to_string(), 
            typing: [Type::Grass, Type::Poison]};

        // Assert Name
        assert_eq!(&correctSpecies.getName(), &parsedSpecies.getName());
        // Assert Species ID
        assert_eq!(&correctSpecies.getIndex(), &parsedSpecies.getIndex());
        // Assert Pokedex
        assert_eq!(&correctSpecies.getPokedex(), &parsedSpecies.getPokedex());
    }

}