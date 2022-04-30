use bevy::utils::{HashMap, HashSet};
use rand::Rng;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
  Grass,
  Water,
  Sand,
  Trees,
  Stone,
}

impl TileType {
  pub fn all_types() -> Vec<TileType> {
    return vec![
      TileType::Grass,
      TileType::Water,
      TileType::Sand,
      TileType::Trees,
      TileType::Stone,
    ];
  }

  pub fn random() -> TileType {
    let types = TileType::all_types();
    let index = rand::thread_rng().gen_range(0..types.len());
    types.get(index).unwrap().clone()
  }

  pub fn get_texture(tile_type: &TileType) -> String {
    match tile_type {
      TileType::Grass => "tiles/Grass.png".to_string(),
      TileType::Water => "tiles/Water.png".to_string(),
      TileType::Sand => "tiles/Sand.png".to_string(),
      TileType::Trees => [
        "tiles/Trees01.png".to_string(),
        "tiles/Trees02.png".to_string(),
      ]
      .get(rand::thread_rng().gen_range(0..2))
      .unwrap()
      .clone(),
      TileType::Stone => "tiles/Rock.png".to_string(),
    }
  }

  pub fn default_rules() -> HashMap<TileType, HashSet<TileType>> {
    let mut rules = HashMap::new();
    rules.insert(
      TileType::Grass,
      HashSet::from_iter(vec![
        TileType::Grass,
        TileType::Trees,
        TileType::Sand,
        TileType::Stone,
      ]),
    );
    rules.insert(
      TileType::Water,
      HashSet::from_iter(vec![TileType::Water, TileType::Sand]),
    );
    rules.insert(
      TileType::Sand,
      HashSet::from_iter(vec![TileType::Sand, TileType::Water, TileType::Grass]),
    );
    rules.insert(
      TileType::Trees,
      HashSet::from_iter(vec![TileType::Trees, TileType::Grass]),
    );
    rules.insert(TileType::Stone, HashSet::from_iter(vec![TileType::Grass]));
    return rules;
  }
}
