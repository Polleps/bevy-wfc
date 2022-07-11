use bevy::utils::HashSet;

#[derive(Clone, Debug)]
pub enum Cell {
  Collapsed(String),
  Superposition(HashSet<String>),
}

impl Cell {
  pub fn new(possible_types: &Vec<String>) -> Cell {
    Cell::Superposition(HashSet::from_iter(possible_types.clone()))
  }
}
