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
    let mut encodedText: Vec<u8> = Vec::new();

    for char in encoded {
        match char {
            0x80 => encodedText.push(b'A'),
            0x81 => encodedText.push(b'B'),
            0x82 => encodedText.push(b'C'),
            0x83 => encodedText.push(b'D'),
            0x84 => encodedText.push(b'E'),
            0x85 => encodedText.push(b'F'),
            0x86 => encodedText.push(b'G'),
            0x87 => encodedText.push(b'H'),
            0x88 => encodedText.push(b'I'),
            0x89 => encodedText.push(b'J'),
            0x8A => encodedText.push(b'K'),
            0x8B => encodedText.push(b'L'),
            0x8C => encodedText.push(b'M'),
            0x8D => encodedText.push(b'N'),
            0x8E => encodedText.push(b'O'),
            0x8F => encodedText.push(b'P'),
            0x90 => encodedText.push(b'Q'),
            0x91 => encodedText.push(b'R'),
            0x92 => encodedText.push(b'S'),
            0x93 => encodedText.push(b'T'),
            0x94 => encodedText.push(b'U'),
            0x95 => encodedText.push(b'V'),
            0x96 => encodedText.push(b'W'),
            0x97 => encodedText.push(b'X'),
            0x98 => encodedText.push(b'Y'),
            0x99 => encodedText.push(b'Z'),
            0x9A => encodedText.push(b'('),
            0x9B => encodedText.push(b')'),
            0x9C => encodedText.push(b':'),
            0x9D => encodedText.push(b';'),
            0x9E => encodedText.push(b'['),
            0x9F => encodedText.push(b']'),
            0xA0 => encodedText.push(b'a'),
            0xA1 => encodedText.push(b'b'),
            0xA2 => encodedText.push(b'c'),
            0xA3 => encodedText.push(b'd'),
            0xA4 => encodedText.push(b'e'),
            0xA5 => encodedText.push(b'f'),
            0xA6 => encodedText.push(b'g'),
            0xA7 => encodedText.push(b'h'),
            0xA8 => encodedText.push(b'i'),
            0xA9 => encodedText.push(b'j'),
            0xAA => encodedText.push(b'k'),
            0xAB => encodedText.push(b'l'),
            0xAC => encodedText.push(b'm'),
            0xAD => encodedText.push(b'n'),
            0xAE => encodedText.push(b'o'),
            0xAF => encodedText.push(b'p'),
            0xB0 => encodedText.push(b'q'),
            0xB1 => encodedText.push(b'r'),
            0xB2 => encodedText.push(b's'),
            0xB3 => encodedText.push(b't'),
            0xB4 => encodedText.push(b'u'),
            0xB5 => encodedText.push(b'v'),
            0xB6 => encodedText.push(b'w'),
            0xB7 => encodedText.push(b'x'),
            0xB8 => encodedText.push(b'y'),
            0xB9 => encodedText.push(b'z'),
            0xBA => encodedText.extend_from_slice(&[b'\xC3', b'\xA9']),
            0x50 => encodedText.push(b' '),
            _   => encodedText.push(b' ') 
        }; 
    }

    let decodedText = String::from_utf8(encodedText).unwrap();

    return decodedText;
}

/// Encodes text into the character encoding used by Gen 1
fn textEncode(decoded: &String) -> [i16; 11]{
    let mut encoded: [i16; 11] = [0; 11];

    for index in 0..11 {
        let currChar = decoded.chars().nth(index).unwrap();
        encoded[index] = match currChar {
            'A' => 0x80,
            'B' => 0x81,
            'C' => 0x82,
            'D' => 0x83,
            'E' => 0x84,
            'F' => 0x85,
            'G' => 0x86,
            'H' => 0x87,
            'I' => 0x88,
            'J' => 0x89,
            'K' => 0x8A,
            'L' => 0x8B,
            'M' => 0x8C,
            'N' => 0x8D,
            'O' => 0x8E,
            'P' => 0x8F,
            'Q' => 0x90,
            'R' => 0x91,
            'S' => 0x92,
            'T' => 0x93,
            'U' => 0x94,
            'V' => 0x95,
            'W' => 0x96,
            'X' => 0x97,
            'Y' => 0x98,
            'Z' => 0x99,
            '(' => 0x9A,
            ')' => 0x9B,
            ':' => 0x9C,
            ';' => 0x9D,
            '[' => 0x9E,
            ']' => 0x9F,
            'a' => 0xA0,
            'b' => 0xA1,
            'c' => 0xA2,
            'd' => 0xA3,
            'e' => 0xA4,
            'f' => 0xA5,
            'g' => 0xA6,
            'h' => 0xA7,
            'i' => 0xA8,
            'j' => 0xA9,
            'k' => 0xAA,
            'l' => 0xAB,
            'm' => 0xAC,
            'n' => 0xAD,
            'o' => 0xAE,
            'p' => 0xAF,
            'q' => 0xB0,
            'r' => 0xB1,
            's' => 0xB2,
            't' => 0xB3,
            'u' => 0xB4,
            'v' => 0xB5,
            'w' => 0xB6,
            'x' => 0xB7,
            'y' => 0xB8,
            'z' => 0xB9,
            'é' => 0xBA,
            ' ' => 0x50,
            _   => 0x50 
        }
    }

    return encoded;
}

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
        let mut letterList: [i16; 11] = [0x50; 11];
        letterList[0] = 0x8F;
        assert_eq!(textDecode(&letterList),"P          ");
    }

    #[test]
    fn textDecode_CapitalWord() {
        // Word "POKEMON"
        let word: [i16; 11] = [0x8F, 0x8E, 0x8A, 0x84, 0x8C, 0x8E, 0x8D, 0x50, 0x50, 0x50, 0x50];
        assert_eq!(textDecode(&word), "POKEMON    ");
    }

    #[test]
    fn textDecode_LowercaseLetter() {
        // Letter "p"
        let mut letterList: [i16; 11] = [0x50; 11];
        letterList[0] = 0xAF;
        assert_eq!(textDecode(&letterList), "p          ");
    }

    #[test]
    fn textDecode_LowercaseWord() {
        // Word "pokemon"
        let word: [i16; 11] = [0xAF, 0xAE, 0xAA, 0xA4, 0xAC, 0xAE, 0xAD, 0x50, 0x50, 0x50, 0x50];
        assert_eq!(textDecode(&word), "pokemon    ");
    }

    #[test]
    fn textDecode_MixedcaseWord() {
        // Word "Pokemon"
        let word: [i16; 11] = [0x8F, 0xAE, 0xAA, 0xA4, 0xAC, 0xAE, 0xAD, 0x50, 0x50, 0x50, 0x50];
        assert_eq!(textDecode(&word), "Pokemon    ");
    }

    #[test]
    fn textDecode_SpecialChar() {
        // Word "Pokemon"
        let word: [i16; 11] = [0x8F, 0xAE, 0xAA, 0xBA, 0xAC, 0xAE, 0xAD, 0x50, 0x50, 0x50, 0x50];
        assert_eq!(textDecode(&word), "Pokémon    ");
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

    #[test]
    fn integrityCheck_Incorrect_PkmnCrystal() {

        let filePath = "./test/POKEMON CRYSTAL.sav";
        let saveFile = fs::read(filePath).unwrap();
        let fileHasIntegrity = integrityCheck(&saveFile);

        assert_eq!(fileHasIntegrity, false);
    }

    #[test]
    fn textEncode_UppercaseLetter() {
        // Letter "P"
        let decodedText = String::from("P          ");

        let mut letterList: [i16; 11] = [0x50; 11];
        letterList[0] = 0x8F;
        assert_eq!(textEncode(&decodedText), letterList);
    }

    #[test]
    fn textEncode_CapitalWord() {
        // Word "POKEMON"
        let decodedText = String::from("POKEMON    ");
        let word: [i16; 11] = [0x8F, 0x8E, 0x8A, 0x84, 0x8C, 0x8E, 0x8D, 0x50, 0x50, 0x50, 0x50];

        assert_eq!(textEncode(&decodedText), word);
    }

    #[test]
    fn textEncode_LowercaseLetter() {
        // Letter "p"
        let decodedText = String::from("p          ");

        let mut letterList: [i16; 11] = [0x50; 11];
        letterList[0] = 0xAF;
        assert_eq!(textEncode(&decodedText), letterList);
    }

    #[test]
    fn textEncode_LowercaseWord() {
        // Word "pokemon"
        let decodedText = String::from("pokemon    ");

        let word: [i16; 11] = [0xAF, 0xAE, 0xAA, 0xA4, 0xAC, 0xAE, 0xAD, 0x50, 0x50, 0x50, 0x50];
        assert_eq!(textEncode(&decodedText), word);
    }

    #[test]
    fn textEncode_MixedcaseWord() {
        // Word "Pokemon"
        let decodedText = String::from("Pokemon    ");

        let word: [i16; 11] = [0x8F, 0xAE, 0xAA, 0xA4, 0xAC, 0xAE, 0xAD, 0x50, 0x50, 0x50, 0x50];
        assert_eq!(textEncode(&decodedText), word);
    }
    
    #[test]
    fn textEncode_SpecialChar() {
        // Word "Pokemon"
        let decodedText = String::from("Pokémon    ");

        let word: [i16; 11] = [0x8F, 0xAE, 0xAA, 0xBA, 0xAC, 0xAE, 0xAD, 0x50, 0x50, 0x50, 0x50];
        assert_eq!(textEncode(&decodedText), word);
    }

}