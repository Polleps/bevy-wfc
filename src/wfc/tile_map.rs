use std::{cmp::Ordering, collections::VecDeque};

use bevy::utils::{HashMap, HashSet};

use super::{
  cell::Cell,
  tile_rules::{Direction, TileRules},
};
use rand::Rng;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug)]
enum Validity {
  Valid,
  Invalid,
  Impossible,
}

pub enum MapStatus {
  Generating,
  Finished,
}

#[derive(Debug)]
pub struct TileMap {
  pub width: i32,
  pub height: i32,
  pub tiles: HashMap<Position, Cell>,
  pub rules: TileRules,
}

impl TileMap {
  /**
   * Creates new TileMap with the given width and height.
   * All tiles are empty.
   */
  pub fn new(width: i32, height: i32, rules: TileRules) -> TileMap {
    TileMap {
      width,
      height,
      tiles: HashMap::new(),
      rules,
    }
  }

  fn get_neighbour(&self, position: &Position, direction: &Direction) -> Option<(Position, Cell)> {
    let mut new_position = Position {
      x: position.x,
      y: position.y,
    };

    match direction {
      Direction::North => new_position.y -= 1,
      Direction::East => new_position.x += 1,
      Direction::South => new_position.y += 1,
      Direction::West => new_position.x -= 1,
    }

    if new_position.x < 0
      || new_position.y < 0
      || new_position.x >= self.width
      || new_position.y >= self.height
    {
      return None;
    }

    let cell = self.tiles.get(&new_position)?;

    Some((new_position, cell.clone()))
  }

  fn valid_neighbour(&self, a: &str, b: &Cell, direction: &Direction) -> Validity {
    match b {
      Cell::Collapsed(n_type) => {
        let result = self.rules.valid_neighbour(a, n_type, direction);

        if result {
          Validity::Valid
        } else {
          Validity::Impossible
        }
      }
      Cell::Superposition(n_types) => {
        let result = n_types
          .iter()
          .any(|n_type| self.rules.valid_neighbour(a, n_type, direction));

        if result {
          Validity::Valid
        } else {
          Validity::Invalid
        }
      }
    }
  }

  fn init_tiles(&mut self) {
    for x in 0..self.width {
      for y in 0..self.height {
        self
          .tiles
          .insert(Position { x, y }, Cell::new(&self.rules.tile_types));
      }
    }
  }

  fn get_all_neighbours(&self, position: &Position) -> Vec<(Direction, Position, Cell)> {
    let directions = [
      Direction::North,
      Direction::East,
      Direction::South,
      Direction::West,
    ];

    let neighbours: Vec<(Direction, Position, Cell)> = directions
      .iter()
      .filter_map(|direction| {
        let (pos, cell) = self.get_neighbour(position, direction)?;
        Some((direction.clone(), pos, cell))
      })
      .collect();

    neighbours
  }

  /**
   * Try to collapse cell.
   * Returns positions of the cells neighbours if the cell was changed in some way.
   */
  fn update_cell(&mut self, position: Position) -> Option<Vec<Position>> {
    let types = match self.tiles.get(&position).unwrap() {
      Cell::Collapsed(_) => {
        // The cell is already collapsed, it doesn't need to update.
        return None;
      }
      Cell::Superposition(tiles) => tiles.clone(),
    };

    let neighbours = self.get_all_neighbours(&position);

    let type_filter = |tile_type: &&std::string::String| {
      // Fold neighgours to find out if the tiletype can exist next to its neighbours.
      let validity = neighbours
        .iter()
        .fold(Validity::Invalid, |acc, (direction, _, item)| {
          if let Validity::Impossible = acc {
            // A collapsed tile next to this cell is an incompatible neighbour.
            return Validity::Impossible;
          }

          match self.valid_neighbour(tile_type, item, direction) {
            // The neighbour has a valid tile type for this type.
            Validity::Valid => Validity::Valid,
            // The neighbour is in superposition but none of its possible states are valid with this type.
            Validity::Invalid => acc,
            // The neighbour is collapsed and its type is not a valid neighbour for this one.
            Validity::Impossible => Validity::Impossible,
          }
        });

      matches!(validity, Validity::Valid)
    };

    let possible_types: Vec<String> = types.iter().filter(type_filter).cloned().collect();

    if possible_types.len() == 1 {
      // Cell has only one possible type left so we collapse the cell.
      self.tiles.insert(
        position,
        Cell::Collapsed(possible_types.first().unwrap().clone()),
      );
    } else {
      // Cell has more than one possible type left so we update the cell to reflect these changes.
      let set: HashSet<String> = possible_types.iter().cloned().collect();

      self.tiles.insert(position, Cell::Superposition(set));
    }

    // If the cell has been changed in some way we return the positions of the neighbours.
    if possible_types.len() != types.len() {
      let mut changed_positions = Vec::new();
      for (_, pos, _) in neighbours {
        changed_positions.push(pos);
      }

      return Some(changed_positions);
    }

    None
  }

  fn calculate_entropy(&self, types: &HashSet<String>) -> i32 {
    if types.len() == self.rules.tile_types.len() {
      return i32::MAX;
    }

    types
      .iter()
      .fold(0, |acc, item| acc + self.rules.get_weight_of_type(item))
  }

  /**
   * A function that finds the tile with the lowest amount of possible types
   * If multiple tiles have the same amount of possible types, it will choose one at random.
   */
  fn find_lowest_entropy(&self) -> Option<Position> {
    let mut lowest_tiles: Vec<Position> = Vec::new();
    let mut lowest_entropy = std::i32::MAX;
    let mut rng = rand::thread_rng();

    for (position, cell) in self.tiles.iter() {
      if let Cell::Superposition(types) = cell {
        let entropy = self.calculate_entropy(types);

        match entropy.cmp(&lowest_entropy) {
          Ordering::Less => {
            lowest_tiles = Vec::from([position.clone()]);
            lowest_entropy = entropy;
          }
          Ordering::Equal => {
            lowest_tiles.push(position.clone());
          }
          Ordering::Greater => {}
        }
      }
    }

    if lowest_tiles.is_empty() {
      return None;
    }

    let index: usize = rng.gen_range(0..lowest_tiles.len());
    Some(lowest_tiles.get(index)?.clone())
  }

  /**
   * Collapses the cell with the lowest entropy and returns its position.
   */
  fn collapse_to_random_type(&mut self) -> Option<Position> {
    let position = self.find_lowest_entropy()?;
    let cell = self.tiles.get(&position).unwrap().clone();

    match cell {
      Cell::Collapsed(_) => panic!("Tried to collapse a collapsed cell"),
      Cell::Superposition(types) => {
        let type_to_collapse = &self.rules.random_tile_from_set(&types);

        self
          .tiles
          .insert(position.clone(), Cell::Collapsed(type_to_collapse.clone()));
      }
    }

    Some(position)
  }

  pub fn update_and_propagate(&mut self) -> MapStatus {
    let mut updated_positions = VecDeque::new();

    match self.collapse_to_random_type() {
      Some(position) => {
        self
          .get_all_neighbours(&position)
          .iter()
          .for_each(|(_, pos, _)| updated_positions.push_back(pos.clone()));
      }
      None => return MapStatus::Finished,
    }

    while !updated_positions.is_empty() {
      if let Some(positions) = self.update_cell(updated_positions.pop_front().expect("wtf")) {
        for position in positions {
          updated_positions.push_back(position);
        }
      }
    }

    MapStatus::Generating
  }

  pub fn generate(&mut self) {
    self.init_tiles();

    loop {
      if let MapStatus::Finished = self.update_and_propagate() {
        break;
      }
    }

    self.remove_sand_islands();
  }

  pub fn clear(&mut self) {
    self.init_tiles();
  }

  fn should_remove_sand(&self, position: &Position) -> bool {
    let mut surrounding_tiles: Vec<Cell> = Vec::new();

    for x in -1..2 {
      for y in -1..2 {
        if x == 0 && y == 0 {
          continue;
        }

        let position = Position {
          x: position.x + x,
          y: position.y + y,
        };

        if let Some(tile) = self.tiles.get(&position) {
          surrounding_tiles.push(tile.clone());
        }
      }
    }

    let grass = "grass".to_string();

    let should_replace = !surrounding_tiles
      .iter()
      .any(|cell| matches!(cell, Cell::Collapsed(tile) if tile == &grass));

    should_replace
  }

  pub fn remove_sand_islands(&mut self) {
    let sand = "sand".to_string();

    let mut cells_to_update = Vec::new();

    for (position, cell) in self.tiles.iter() {
      if matches!(cell, Cell::Collapsed(tile) if tile == &sand) && self.should_remove_sand(position)
      {
        cells_to_update.push(position.clone());
      }
    }

    let water = "water".to_string();

    for cell in cells_to_update {
      self.tiles.insert(cell, Cell::Collapsed(water.clone()));
    }
  }
}
