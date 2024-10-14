#[derive(Debug)]
#[derive(PartialEq)]
pub enum Type {
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

    // For single Type pokemon
    Null
}
impl Type {
    pub fn get(index: &i16) -> Type{
        let returnPkmnType = match index {
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

        returnPkmnType
    }
}
