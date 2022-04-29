use rand::Rng;

pub enum NeighbourValidity {
  Valid,
}

#[derive(Clone, PartialEq, Eq, Hash)]
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
}
