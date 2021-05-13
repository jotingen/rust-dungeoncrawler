use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Abilities {
    pub strength: u32,
    pub dexterity: u32,
    pub charisma: u32,
    pub constitution: u32,
    pub intellect: u32,
    pub wisdom: u32,
}

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
