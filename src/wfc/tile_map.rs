use std::collections::VecDeque;

use bevy::utils::{HashMap, HashSet};

use super::{
  cell::Cell,
  tile_type::{TileRules, TileType},
};
use rand::Rng;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

enum Direction {
  North,
  East,
  South,
  West,
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

pub struct TileMap {
  pub width: i32,
  pub height: i32,
  pub tiles: HashMap<Position, Cell>,
  pub rules: TileRules,
}

impl TileMap {
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

  fn valid_neighbour(&self, a: &TileType, b: &Cell) -> Validity {
    let rules = self.rules.adjacency.clone();

    match b {
      Cell::Collapsed(n_type) => {
        let result = rules.get(n_type).unwrap().contains(a);

        if result {
          Validity::Valid
        } else {
          Validity::Impossible
        }
      }
      Cell::Superposition(n_types) => {
        let result = n_types
          .iter()
          .any(|n_type| rules.get(n_type).unwrap().contains(a));

        if result {
          Validity::Valid
        } else {
          Validity::Invalid
        }
      }
    }
  }

  fn init_tiles(width: i32, height: i32) -> HashMap<Position, Cell> {
    let mut tiles = HashMap::new();

    for x in 0..width {
      for y in 0..height {
        tiles.insert(Position { x, y }, Cell::new());
      }
    }

    tiles
  }
  /**
   * Creates new TileMap with the given width and height.
   * The map is filled with all cells in superposition.
   */
  pub fn new(width: i32, height: i32, rules: TileRules) -> TileMap {
    let tiles = TileMap::init_tiles(width, height);

    TileMap {
      width,
      height,
      tiles,
      rules,
    }
  }

  fn get_all_neighbours(&self, position: &Position) -> Vec<(Position, Cell)> {
    let directions = [
      Direction::North,
      Direction::East,
      Direction::South,
      Direction::West,
    ];

    let neighbours = directions
      .iter()
      .filter_map(|direction| self.get_neighbour(position, direction))
      .collect();

    return neighbours;
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

    let type_filter = |tile_type: &&TileType| {
      // Fold neighgours to find out if the tiletype can exist next to its neighbours.
      let validity = neighbours.iter().fold(Validity::Invalid, |acc, (_, item)| {
        if let Validity::Impossible = acc {
          // A collapsed tile next to this cell is an incompatible neighbour.
          return Validity::Impossible;
        }

        match self.valid_neighbour(tile_type, item) {
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

    let possible_types: Vec<TileType> = types.iter().filter(type_filter).cloned().collect();

    if possible_types.len() == 1 {
      self.tiles.insert(
        position.clone(),
        Cell::Collapsed(possible_types.first().unwrap().clone()),
      );
    } else {
      let set: HashSet<TileType> = possible_types.iter().cloned().collect();

      self
        .tiles
        .insert(position.clone(), Cell::Superposition(set));
    }

    if possible_types.len() != types.len() {
      return Some(
        neighbours
          .iter()
          .filter_map(|tup: &(Position, Cell)| Some(tup.0.clone()))
          .collect(),
      );
    }

    return None;
  }

  /**
   * A function that finds the tile with the lowest amount of possible types
   * If multiple tiles have the same amount of possible types, it will choose one at random.
   */
  fn find_lowest_entropy(&self) -> Option<Position> {
    let mut lowest_tiles: Vec<Position> = Vec::new();
    let mut lowest_count = std::usize::MAX;
    let mut rng = rand::thread_rng();

    for (position, cell) in self.tiles.iter() {
      match cell {
        Cell::Collapsed(_) => continue,
        Cell::Superposition(types) => {
          if types.len() < lowest_count {
            lowest_tiles = Vec::new();
            lowest_tiles.push(position.clone());
            lowest_count = types.len();
          } else if types.len() == lowest_count {
            lowest_tiles.push(position.clone());
          }
        }
      }
    }

    if lowest_tiles.len() == 0 {
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
        let type_to_collapse = TileType::random_from_set(&types, &self.rules);

        self
          .tiles
          .insert(position.clone(), Cell::Collapsed(type_to_collapse));
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
          .for_each(|(pos, _)| updated_positions.push_back(pos.clone()));
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
    loop {
      if let MapStatus::Finished = self.update_and_propagate() {
        break;
      }
    }

    self.remove_sand_islands();
  }

  pub fn clear(&mut self) {
    let tiles = TileMap::init_tiles(self.width, self.height);
    self.tiles = tiles;
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

    let should_replace = !surrounding_tiles
      .iter()
      .any(|cell| matches!(cell, Cell::Collapsed(TileType::Grass)));

    return should_replace;
  }

  pub fn remove_sand_islands(&mut self) {
    let mut cells_to_update = Vec::new();

    for (position, cell) in self.tiles.iter() {
      if let Cell::Collapsed(TileType::Sand) = cell {
        if self.should_remove_sand(&position) {
          cells_to_update.push(position.clone());
        }
      }
    }

    for cell in cells_to_update {
      self.tiles.insert(cell, Cell::Collapsed(TileType::Water));
    }
  }
}
