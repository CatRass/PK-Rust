
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

#[derive(Debug)]
pub struct IVs {
    // TODO: Make Getters
    pub hp:  u16,
    pub atk: u16,
    pub def: u16,
    pub spd: u16,
    pub spc: u16
}
impl IVs {
    /// Returns a string to display all stats
    pub fn to_string(&self) -> String {
        return format!("\tATK IV: {}\n\tDEF IV:{}\n\tSPD IV: {}\n\tSPCL IV: {}\n",
                        self.atk,
                        self.def,
                        self.spd,
                        self.spc
                    );
    }
}

#[derive(Debug)]
pub struct EVs {
    // TODO: Make Getters
    pub hp: u16,
    pub atk: u16,
    pub def: u16,
    pub spd: u16,
    pub spc: u16
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
}
