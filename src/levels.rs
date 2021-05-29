use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
enum Tile {
  Empty,
  Stone,
}
impl Default for Tile {
    fn default() -> Self {
        Tile::Stone
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Level{tiles: Vec<Vec<Tile>>}

impl Level {
    fn generate(level_number: usize) -> Level {
      //Generate level dimensions based on level
      //Start small, grow quickly, then stablize
      //y = a / (1 + b e-kx ), k > 0
      let a: f32 = 1000.0;
      let b: f32 = 100.0;
      let k: f32 = 0.5;
      let e: f32 = (1.0 as f32).exp();
      let dimension = (a / ( 1.0 + b * e.powf(-k * level_number as f32))).round() as usize;
      let mut level:Level= Level{tiles: vec![vec![Tile::Stone; dimension]; dimension]};
      level
    }

    pub fn map(&self) -> String {
      let mut map_str = "".to_string();
      for tile_row in self.tiles.iter(){
        for tile in tile_row {
          map_str = format!("{}{}",map_str,"@");
        }
        map_str = format!("{}\n",map_str);
      }
      map_str
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Levels {
  level: Vec<Level>,
}

impl Levels {
    pub fn new() -> Levels {
      Levels {level: Vec::new()}
    }

    pub fn level(&mut self, level_number: usize) -> &Level {
      if level_number >= self.level.len() {
        for number in self.level.len()..level_number+1 {
          let new_level = Level::generate(number);
          self.level.push(new_level)
        }
      }
      &self.level[level_number]
    }
    
}

