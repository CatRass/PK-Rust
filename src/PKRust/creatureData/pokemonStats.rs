use super::super::utils::formatError;


#[derive(Debug)]
pub struct Stats {
    // TODO: Make a Getter for These
    pub hp: u16,
    pub atk: u16,
    pub def: u16,
    pub spd: u16,
    pub spc: u16
}
impl Stats {
    /// Returns a string to display all stats
    pub fn to_string(&self) -> String {
        return format!("\tHP: {}\n\tATK: {}\n\tDEF:{}\n\tSPD: {}\n\tSPCL: {}\n",
                        self.hp,
                        self.atk,
                        self.def,
                        self.spd,
                        self.spc
                    );
    }

}

#[cfg(test)]
mod statsTests {

}

#[derive(Debug)]
pub struct IVs {
    hp:  u16,
    atk: u16,
    def: u16,
    spd: u16,
    spc: u16
}
impl IVs {

    /// Returns a blank IV Struct
    pub fn new() -> IVs {
        return IVs {
            hp: 0,
            atk: 0,
            def: 0,
            spd: 0,
            spc: 0
        };
    }

    /// Returns a string to display all stats
    pub fn to_string(&self) -> String {
        return format!("\tATK IV: {}\n\tDEF IV:{}\n\tSPD IV: {}\n\tSPCL IV: {}\n",
                        self.atk,
                        self.def,
                        self.spd,
                        self.spc
                    );
    }

    // ========   GETTERS   ========
    
    /// Getter for HP IV
    pub fn getHP(&self) -> &u16 {
        return &self.hp;
    }

    /// Getter for ATK IV
    pub fn getATK(&self) -> &u16 {
        return &self.atk;
    }

    /// Getter for DEF IV
    pub fn getDEF(&self) -> &u16 {
        return &self.def;
    }

    /// Getter for SPD IV
    pub fn getSPD(&self) -> &u16 {
        return &self.spd;
    }

    /// Getter for SPC IV
    pub fn getSPC(&self) -> &u16 {
        return &self.spc;
    }


    // ========   SETTERS   ========

    /// Returns a full IV object from an array of values
    pub fn setAll(ivArr: [u16;5]) -> IVs {
        return IVs{hp: ivArr[0], atk: ivArr[1], def: ivArr[2], spd: ivArr[3], spc: ivArr[4]};
    }

    /// Setter for HP IV
    /// 
    /// IV's can only be values 0-15
    pub fn setHP(&mut self, newHP: u16) -> Result<bool, String> {
        
        // First we check if the value is over 15
        if newHP > 15 {
            return Err(formatError(format!("HP IV value is \"{}\", which is over max value 15", newHP)));
        }

        // If it's not, we set the new HP
        self.hp = newHP;

        // And return an Ok
        return Ok(true);

    }

    /// Setter for Attack IV
    /// 
    /// IV's can only be values 0-15
    pub fn setATK(&mut self, newATK: u16) -> Result<bool, String> {
        
        // First we check if the value is over 15
        if newATK > 15 {
            return Err(formatError(format!("ATK IV value is \"{}\", which is over max value 15", newATK)));
        }

        // If it's not, we set the new HP
        self.atk = newATK;

        // And return an Ok
        return Ok(true);
    }

    /// Setter for Defence IV
    /// 
    /// IV's can only be values 0-15
    pub fn setDEF(&mut self, newDEF: u16) -> Result<bool, String> {
         
        // First we check if the value is over 15
        if newDEF > 15 {
            return Err(formatError(format!("DEF IV value is \"{}\", which is over max value 15", newDEF)));
        }

        // If it's not, we set the new HP
        self.def = newDEF;

        // And return an Ok
        return Ok(true);
    }

    /// Setter for Speed IV
    /// 
    /// IV's can only be values 0-15
    pub fn setSPD(&mut self, newSPD: u16) -> Result<bool, String> {
                 
        // First we check if the value is over 15
        if newSPD > 15 {
            return Err(formatError(format!("SPD IV value is \"{}\", which is over max value 15", newSPD)));
        }

        // If it's not, we set the new HP
        self.spd = newSPD;

        // And return an Ok
        return Ok(true);
    }

    /// Setter for Special IV
    /// 
    /// IV's can only be values 0-15
    pub fn setSPC(&mut self, newSPC: u16) -> Result<bool, String> {
        // First we check if the value is over 15
        if newSPC > 15 {
            return Err(formatError(format!("SPC IV value is \"{}\", which is over max value 15", newSPC)));
        }

        // If it's not, we set the new HP
        self.spd = newSPC;

        // And return an Ok
        return Ok(true);    
    }

}

#[cfg(test)]
mod IVTests {

    use super::*;

    #[test]
    fn setHP_Correct() {
        let mut testIVs = IVs::new();

        // Boundary value
        let newHP: u16 = 15;

        let changeHPREsult = testIVs.setHP(newHP); 

        assert!(changeHPREsult.is_ok());
        assert_eq!(changeHPREsult.unwrap(), true);

    }

    #[test]
    fn setHP_Incorrect() {
        let mut testIVs = IVs::new();

        // Boundary value is 15, this is over the value
        let newHP: u16 = 16;

        let changeHPREsult = testIVs.setHP(newHP); 

        assert!(changeHPREsult.is_err());
        assert_eq!(changeHPREsult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: HP IV value is \"16\", which is over max value 15");

    }
    
    #[test]
    fn setATK_Correct() {
        let mut testIVs = IVs::new();

        // Boundary value
        let newATK: u16 = 15;

        let changeATKREsult = testIVs.setATK(newATK); 

        assert!(changeATKREsult.is_ok());
        assert_eq!(changeATKREsult.unwrap(), true);

    }

    #[test]
    fn setATK_Incorrect() {
        let mut testIVs = IVs::new();

        // Boundary value is 15, this is over the value
        let newATK: u16 = 16;

        let changeATKREsult = testIVs.setATK(newATK); 

        assert!(changeATKREsult.is_err());
        assert_eq!(changeATKREsult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: ATK IV value is \"16\", which is over max value 15");

    }

    #[test]
    fn setDEF_Correct() {
        let mut testIVs = IVs::new();

        // Boundary value
        let newDEF: u16 = 15;

        let changeDEFREsult = testIVs.setDEF(newDEF); 

        assert!(changeDEFREsult.is_ok());
        assert_eq!(changeDEFREsult.unwrap(), true);

    }

    #[test]
    fn setDEF_Incorrect() {
        let mut testIVs = IVs::new();

        // Boundary value is 15, this is over the value
        let newDEF: u16 = 16;

        let changeDEFREsult = testIVs.setDEF(newDEF); 

        assert!(changeDEFREsult.is_err());
        assert_eq!(changeDEFREsult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: DEF IV value is \"16\", which is over max value 15");

    }

    #[test]
    fn setSPD_Correct() {
        let mut testIVs = IVs::new();

        // Boundary value
        let newSPD: u16 = 15;

        let changeSPDREsult = testIVs.setSPD(newSPD); 

        assert!(changeSPDREsult.is_ok());
        assert_eq!(changeSPDREsult.unwrap(), true);

    }

    #[test]
    fn setSPD_Incorrect() {
        let mut testIVs = IVs::new();

        // Boundary value is 15, this is over the value
        let newSPD: u16 = 16;

        let changeSPDREsult = testIVs.setSPD(newSPD); 

        assert!(changeSPDREsult.is_err());
        assert_eq!(changeSPDREsult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: SPD IV value is \"16\", which is over max value 15");

    }

    #[test]
    fn setSPC_Correct() {
        let mut testIVs = IVs::new();

        // Boundary value
        let newSPC: u16 = 15;

        let changeSPCREsult = testIVs.setSPD(newSPC); 

        assert!(changeSPCREsult.is_ok());
        assert_eq!(changeSPCREsult.unwrap(), true);

    }

    #[test]
    fn setSPC_Incorrect() {
        let mut testIVs = IVs::new();

        // Boundary value is 15, this is over the value
        let newSPD: u16 = 16;

        let changeSPCREsult = testIVs.setSPC(newSPD); 

        assert!(changeSPCREsult.is_err());
        assert_eq!(changeSPCREsult.unwrap_err(), "\u{1b}[0;31mError\u{1b}[0m: SPC IV value is \"16\", which is over max value 15");

    }


}

#[derive(Debug)]
pub struct EVs {
    // TODO: Make Getters
    hp: u16,
    atk: u16,
    def: u16,
    spd: u16,
    spc: u16
}
impl EVs {
    /// Returns a string to display all EVs
    pub fn to_string(&self) -> String {
        return format!("\tHP EV: {}\n\tATK EV: {}\n\tDEF EV:{}\n\tSPD EV: {}\n\tSPCL EV: {}\n",
                        self.hp,
                        self.atk,
                        self.def,
                        self.spd,
                        self.spc
                    );
    }

    /// Returns a blank EV Struct
    pub fn new() -> EVs {
        return EVs {
            hp: 0,
            atk: 0,
            def: 0,
            spd: 0,
            spc: 0
        };
    }
    
    // ========   GETTERS   ========
    
    /// Getter for HP EV
    pub fn getHP(&self) -> &u16 {
        return &self.hp;
    }

    /// Getter for ATK EV
    pub fn getATK(&self) -> &u16 {
        return &self.atk;
    }

    /// Getter for DEF EV
    pub fn getDEF(&self) -> &u16 {
        return &self.def;
    }

    /// Getter for SPD EV
    pub fn getSPD(&self) -> &u16 {
        return &self.spd;
    }

    /// Getter for SPC EV
    pub fn getSPC(&self) -> &u16 {
        return &self.spc;
    }

    // ========   SETTERS   ========

    /// Returns a full EV object from an array of values
    pub fn setAll(evArr: [u16;5]) -> EVs {
        return EVs{hp: evArr[0], atk: evArr[1], def: evArr[2], spd: evArr[3], spc: evArr[4]};
    }

    /// Setter for HP EV
    /// 
    /// The limit for an EV is 65_535, so input validation
    /// is done outside this function.
    pub fn setHP(&mut self, newHP: u16) {
        self.hp = newHP;
    }

    /// Setter for Attack EV
    /// 
    /// The limit for an EV is 65_535, so input validation
    /// is done outside this function.
    pub fn setATK(&mut self, newATK: u16) {
        self.atk = newATK;
    }

    /// Setter for Defence EV
    /// 
    /// The limit for an EV is 65_535, so input validation
    /// is done outside this function.
    pub fn setDEF(&mut self, newDEF: u16) {
        self.def = newDEF;
    }

    /// Setter for Speed EV
    /// 
    /// The limit for an EV is 65_535, so input validation
    /// is done outside this function.
    pub fn setSPD(&mut self, newSPD: u16) {
        self.spd = newSPD;
    }

    /// Setter for Special EV
    /// 
    /// The limit for an EV is 65_535, so input validation
    /// is done outside this function.
    pub fn setSPC(&mut self, newSPC: u16) {
        self.spc = newSPC;
    }

}

#[cfg(test)]
mod EVTests {

    use super::*;

   #[test]
   fn setHP_Correct() {

        let mut testEVs = EVs::new();

        let newHP: u16 = 65_535;

        testEVs.setHP(newHP);

        assert_eq!(testEVs.getHP(), &newHP);

   }

   #[test]
   fn setATK_Correct() {

        let mut testEVs = EVs::new();

        let newATK: u16 = 65_535;

        testEVs.setATK(newATK);

        assert_eq!(testEVs.getATK(), &newATK);

   }

   #[test]
   fn setDEF_Correct() {

        let mut testEVs = EVs::new();

        let newDEF: u16 = 65_535;

        testEVs.setDEF(newDEF);

        assert_eq!(testEVs.getDEF(), &newDEF);

   }

   #[test]
   fn setSPD_Correct() {

        let mut testEVs = EVs::new();

        let newSPD: u16 = 65_535;

        testEVs.setSPD(newSPD);

        assert_eq!(testEVs.getSPD(), &newSPD);

   }

   #[test]
   fn setSPC_Correct() {

        let mut testEVs = EVs::new();

        let newSPC: u16 = 65_535;

        testEVs.setSPC(newSPC);

        assert_eq!(testEVs.getSPC(), &newSPC);

   }

}
