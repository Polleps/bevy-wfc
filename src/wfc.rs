mod tile_type;

mod wfc {
  use std::collections::{HashMap, HashSet};

  use crate::wfc::tile_type::TileType;

  #[derive(PartialEq, Eq, Hash, Clone)]
  pub struct Position {
    x: i32,
    y: i32,
  }

  enum Direction {
    North,
    East,
    South,
    West,
  }

  enum Validity {
    Valid,
    Invalid,
    Impossible,
  }

  pub enum Cell {
    Collapsed(TileType),
    Superposition(HashSet<TileType>),
  }

  pub struct TileMap {
    pub width: i32,
    pub height: i32,
    pub tiles: HashMap<Position, Cell>,
    pub rules: HashMap<TileType, HashSet<TileType>>,
  }

  impl Cell {
    pub fn new() -> Cell {
      return Cell::Superposition(HashSet::from_iter(TileType::all_types()));
    }
  }

  impl TileMap {
    fn get_neighbour(
      &self,
      position: &Position,
      direction: &Direction,
    ) -> Option<(Position, &Cell)> {
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

      return Some((new_position, cell));
    }

    fn valid_neighbour(&self, a: &TileType, b: &Cell) -> Validity {
      match b {
        Cell::Collapsed(n_type) => {
          let result = self.rules.get(n_type).unwrap().contains(a);

          if result {
            return Validity::Valid;
          } else {
            return Validity::Impossible;
          }
        }
        Cell::Superposition(n_types) => {
          let result = n_types
            .iter()
            .any(|n_type| self.rules.get(n_type).unwrap().contains(a));

          if result {
            return Validity::Valid;
          } else {
            return Validity::Invalid;
          }
        }
      }
    }
    /**
     * Creates new TileMap with the given width and height.
     * The map is filled with all cells in superposition.
     */
    pub fn new(width: i32, height: i32, rules: HashMap<TileType, HashSet<TileType>>) -> TileMap {
      let mut tiles = HashMap::new();

      for x in 0..width {
        for y in 0..height {
          tiles.insert(Position { x, y }, Cell::new());
        }
      }

      return TileMap {
        width,
        height,
        tiles,
        rules,
      };
    }

    fn get_all_neighbours(&self, position: &Position) -> Vec<(Position, &Cell)> {
      let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
      ];

      return directions
        .iter()
        .filter_map(|direction| self.get_neighbour(position, direction))
        .collect();
    }

    /**
     * Try to collapse cell.
     * Returns positions of the cells neighbours if the cell was changed in some way.
     */
    fn update_cell(&mut self, position: &Position) -> Option<Vec<Position>> {
      let types = match self.tiles.get(position).unwrap() {
        Cell::Collapsed(_) => {
          // The cell is already collapsed, it doesn't need to update.
          return None;
        }
        Cell::Superposition(tiles) => tiles.clone(),
      };

      let neighbours = self.get_all_neighbours(position);

      let type_filter = |tile_type: &&TileType| {
        // Fold neighgours to find out if the tiletype can exist next to its neighbours.
        let validity = neighbours.iter().fold(Validity::Invalid, |acc, (_, item)| {
          if let Validity::Impossible = acc {
            // A collapsed tile next to this cell is an incompatible neighbour.
            return Validity::Impossible;
          }

          match self.valid_neighbour(tile_type.clone(), item) {
            // The neighbour has a valid tile type for this type.
            Validity::Valid => Validity::Valid,
            // The neighbour is in superposition but none of its possible states are valid with this type.
            Validity::Invalid => acc,
            // The neighbour is collapsed and its type is not a valid neighbour for this one.
            Validity::Impossible => Validity::Impossible,
          }
        });

        match validity {
          Validity::Valid => true,
          _ => false,
        }
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

      return Some(
        neighbours
          .iter()
          .filter_map(|tup| Some(tup.0.clone()))
          .collect(),
      );
    }

    // A function that finds all cells with a given TileType
    fn get_cells_with_type(&self, tile_type: &TileType) -> Vec<Position> {
      let mut result = Vec::new();

      for (position, cell) in self.tiles.iter() {
        match cell {
          Cell::Collapsed(n_type) => {
            if n_type == tile_type {
              result.push(position.clone());
            }
          }
          Cell::Superposition(n_types) => {
            if n_types.contains(tile_type) {
              result.push(position.clone());
            }
          }
        }
      }

      return result;
    }
  }
}
