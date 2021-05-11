use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Abilities {
    strength: u32,
    dexterity: u32,
    charisma: u32,
    constitution: u32,
    intellect: u32,
    wisdom: u32,
}

#[derive(Serialize, Deserialize, Debug)]
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
