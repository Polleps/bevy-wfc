use bevy::utils::HashSet;

use super::tile_type::TileType;

#[derive(Clone, Debug)]
pub enum Cell {
  Collapsed(TileType),
  Superposition(HashSet<TileType>),
}

impl Cell {
  pub fn new() -> Cell {
    Cell::Superposition(HashSet::from_iter(TileType::all_types()))
  }
}
