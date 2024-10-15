// General Starting Addresses
pub const MONEY_ADDR:       usize   = 0x25F3;
pub const ID_ADDR:          usize   = 0x2605;
pub const NAME_ADDR:        usize   = 0x2598;
pub const PARTY_ADDR:       usize   = 0x2F2C;
pub const PC_ADDR:          usize   = 0x4000;

// PC Offsets

/// Pokemon Species Index
pub const PC_PKMN_OFF:      usize   = 0x16;
/// Original Trainer Name
pub const PC_TRAINER_OFF:   usize   = 0x2AA;
/// Pokemon Nickname Offset
pub const PC_NICK_OFF:      usize   = 0x386;

// Pokemon Data Offsets
pub const NICK_OFF:         usize   = 0x152;
pub const HP_OFF:           usize   = 0x01;
pub const MOVE_OFF:         usize   = 0x08;
pub const PP_OFF:           usize   = 0x1D;
pub const OT_OFF:           usize   = 0x0C;
pub const OTN_OFF:          usize   = 0x110;
pub const EV_OFF:           usize   = 0x11;
pub const STAT_OFF:         usize   = 0x22;
pub const IV_OFF:           usize   = 0x1B;