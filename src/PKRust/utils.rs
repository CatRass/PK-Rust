use super::addresses::{CHECKSUM_END_ADDR, CHECKSUM_RESULT, CHECKSUM_START_ADDR};
use std::num::Wrapping;

pub fn splitByte(unsignedByte: &u8) -> (u8, u8) {
    let byteStr = format!("{:08b}",unsignedByte);
    let (firstByteStr, secondByteStr) = byteStr.split_at(4);
    let firstByte:u8 = u8::from_str_radix(firstByteStr, 2).unwrap();
    let secondByte:u8 = u8::from_str_radix(secondByteStr, 2).unwrap();
    
    return (firstByte, secondByte);
}

/// Decodes text, as text in most games uses character encoding
/// **TODO**: Optimise this too. Yanderedev levels of code
pub fn textDecode(encoded: &[i16; 11]) -> String{
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

pub fn integrityCheck(saveFile: &Vec<u8>) -> bool {
    // We will be using the "easy" way, as shown in the
    // Checksum section of the bulbapedia https://m.bulbapedia.bulbagarden.net/wiki/Save_data_structure_(Generation_I)#Checksum

    // We also need to use "Wrapping" here because the checksum relies
    // on integer over and underflow, but Rust does not allow that on default
    // integers due to type safety.
    let mut checksumVal: Wrapping<u8> = Wrapping(0);
    
    // We use the main data checksum
    // https://m.bulbapedia.bulbagarden.net/wiki/Save_data_structure_(Generation_I)#bank1_checksum
    let checksumRes: Wrapping<u8> = Wrapping(saveFile[CHECKSUM_RESULT]);

    for byte in  CHECKSUM_START_ADDR..CHECKSUM_END_ADDR {
        checksumVal += &saveFile[byte];
    }

    return !checksumVal == checksumRes;
    
}

// ================ TESTS ================ 
#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn textDecode_UppercaseLetter() {
        // Letter "P"
        let mut letterList: [i16; 11] = [0x00; 11];
        letterList[0] = 0x8F;
        assert_eq!(textDecode(&letterList),"P          ");
    }

    #[test]
    fn textDecpde_CapitalWord() {
        // Word "POKEMON"
        let word: [i16; 11] = [0x8F, 0x8E, 0x8A, 0x84, 0x8C, 0x8E, 0x8D, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(textDecode(&word), "POKEMON    ");
    }

    #[test]
    fn textDecode_LowercaseLetter() {
        // Letter "p"
        let mut letterList: [i16; 11] = [0x00; 11];
        letterList[0] = 0xAF;
        assert_eq!(textDecode(&letterList), "p          ");
    }

    #[test]
    fn textDecode_LowercaseWord() {
        // Word "pokemon"
        let word: [i16; 11] = [0xAF, 0xAE, 0xAA, 0xA4, 0xAC, 0xAE, 0xAD, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(textDecode(&word), "pokemon    ");
    }

    #[test]
    fn textDecode_MixedcaseWord() {
        // Word "Pokemon"
        let word: [i16; 11] = [0x8F, 0xAE, 0xAA, 0xA4, 0xAC, 0xAE, 0xAD, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(textDecode(&word), "Pokemon    ");
    }

    #[test]
    fn integrityCheck_Correct_PkmnYellow() {
        let testFiles:Vec<&str> = vec!["./test/POKEMON YELLOW.sav", "./test/POKEMON YELLOW 2.sav"];

        for filePath in testFiles {
            let saveFile = fs::read(filePath).unwrap();
            let fileHasIntegrity = integrityCheck(&saveFile);
    
            assert_eq!(fileHasIntegrity, true);
        }
    }

    #[test]
    fn integrityCheck_Correct_PkmnBlue() {

        let testFiles:Vec<&str> = vec!["./test/POKEMON BLUE.sav", "./test/Pokeblue.sav"];

        for filePath in testFiles {
            let saveFile = fs::read(filePath).unwrap();
            let fileHasIntegrity = integrityCheck(&saveFile);
    
            assert_eq!(fileHasIntegrity, true);
        }

    }

}