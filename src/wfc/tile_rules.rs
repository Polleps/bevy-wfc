use bevy::{
  reflect::TypeUuid,
  utils::{HashMap, HashSet},
};
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AdjacencyRule {
  pub north: HashSet<String>,
  pub east: HashSet<String>,
  pub south: HashSet<String>,
  pub west: HashSet<String>,
}

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "e482a821-2d5e-42d4-9307-912b4fdc825a"]
pub struct TileRules {
  #[serde(rename = "tileTypes")]
  pub tile_types: Vec<String>,
  adjacency: HashMap<String, AdjacencyRule>,
  weights: HashMap<String, i32>,
  indexes: HashMap<String, Vec<i32>>,
}

#[derive(Clone, Debug)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

impl TileRules {
  pub fn empty() -> TileRules {
    TileRules {
      tile_types: vec![],
      adjacency: HashMap::new(),
      weights: HashMap::new(),
      indexes: HashMap::new(),
    }
  }
  /**
   * Get random tile from the given set based on the tiles weight.
   */
  pub fn random_tile_from_set(&self, set: &HashSet<String>) -> String {
    let mut tiles = Vec::new();

    for tile in set {
      for _ in 0..*self.weights.get(tile).unwrap_or(&0) {
        tiles.push(tile.clone());
      }
    }

    if tiles.is_empty() {
      return "grass".to_string();
    }

    tiles[rand::thread_rng().gen_range(0..tiles.len())].clone()
  }

  /**
   * Get tile set index from the given tile type, if the tile has multiple indices a random on wil be chosen.
   */
  pub fn get_tile_index(&self, tile_type: &str) -> i32 {
    let indexes = self.indexes.get(tile_type).unwrap();
    indexes[rand::thread_rng().gen_range(0..indexes.len())]
  }

  /**
   * Check if tile_type a can be a neighbour of tile_type b for the given direction
   */
  pub fn valid_neighbour(
    &self,
    tile_type_a: &str,
    tile_type_b: &str,
    direction: &Direction,
  ) -> bool {
    let rules = self.adjacency.get(tile_type_b);

    if let None = rules {
      return false;
    }
    let rules = rules.unwrap();

    let rules = match direction {
      Direction::North => &rules.north,
      Direction::East => &rules.west,
      Direction::South => &rules.south,
      Direction::West => &rules.east,
    };

    rules.contains(tile_type_a)
  }

  pub fn get_weight_of_type(&self, tile_type: &str) -> i32 {
    *self.weights.get(tile_type).unwrap_or(&0)
  }
}
