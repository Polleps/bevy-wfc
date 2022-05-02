use bevy::utils::{HashMap, HashSet};
use rand::Rng;

#[derive(Clone)]
pub struct TileRules {
  pub adjacency: HashMap<TileType, HashSet<TileType>>,
  pub weights: HashMap<TileType, i32>,
}

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

  pub fn random_from_set(set: &HashSet<TileType>, rules: &TileRules) -> TileType {
    let mut types = Vec::new();

    set.iter().for_each(|t| {
      for _ in 0..(rules.weights.get(t).expect("invalid type") + 1) {
        types.push(t.clone());
      }
    });

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

  pub fn default_rules() -> TileRules {
    let mut adjacency_rules = HashMap::new();
    adjacency_rules.insert(
      TileType::Grass,
      HashSet::from_iter(vec![
        TileType::Grass,
        TileType::Trees,
        TileType::Sand,
        TileType::Stone,
      ]),
    );
    adjacency_rules.insert(
      TileType::Water,
      HashSet::from_iter(vec![TileType::Water, TileType::Sand]),
    );
    adjacency_rules.insert(
      TileType::Sand,
      HashSet::from_iter(vec![TileType::Sand, TileType::Water, TileType::Grass]),
    );
    adjacency_rules.insert(
      TileType::Trees,
      HashSet::from_iter(vec![TileType::Trees, TileType::Grass]),
    );
    adjacency_rules.insert(TileType::Stone, HashSet::from_iter(vec![TileType::Grass]));

    let mut weights = HashMap::new();

    weights.insert(TileType::Grass, 5);
    weights.insert(TileType::Water, 1);
    weights.insert(TileType::Sand, 1);
    weights.insert(TileType::Trees, 3);
    weights.insert(TileType::Stone, 3);

    return TileRules {
      adjacency: adjacency_rules,
      weights,
    };
  }
}
