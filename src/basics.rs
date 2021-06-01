use serde::{Deserialize, Serialize};

///Struct containing the 6 different abilities
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Abilities {
    pub strength: u32,
    pub dexterity: u32,
    pub charisma: u32,
    pub constitution: u32,
    pub intellect: u32,
    pub wisdom: u32,
}

///Enum containing the 9 different alignments
///
///Defaults to N
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Alignment {
    LG, //Lawful good
    NG, //Neutral good
    CG, //Chaotic good
    LN, //Lawful neutral
    N,  //Neutral
    CN, //Chaotic neutral
    LE, //Lawful evil
    NE, //Neutral evil
    CE, //Chaotic evil
}
impl Default for Alignment {
    fn default() -> Self {
        Alignment::N
    }
}

///Enum containing 2 genders
///
///Defaults to M
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Gender {
    M, //Male
    F, //Female
}
impl Default for Gender {
    fn default() -> Self {
        Gender::M
    }
}
