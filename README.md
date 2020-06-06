# Maze Tiles

[![Build Status](https://travis-ci.org/marcbreitung/maze-tiles-rust.svg?branch=master)](https://travis-ci.org/marcbreitung/maze-tiles-rust)

A maze contains multiple tiles (red outline), each tile is divided into fields (black outline), which can be walkable. 

## Tile

![a tile](tile.svg)

````rust
use maze_tiles_rust::tile::position::Position;
use maze_tiles_rust::tile::size::Size;
use maze_tiles_rust::tile::field::Field;
use maze_tiles_rust::tile::tile::Tile;

let position = Position::new(0, 0);
let size = Size::new(3, 3);
let fields = vec![
    Field::Ground, Field::Path, Field::Ground,
    Field::Ground, Field::Path, Field::Ground,
    Field::Ground, Field::Path, Field::Ground,
];
let tile = Tile::new(position, size, fields);
````

## Maze

![a maze](maze.svg)

```rust
use maze_tiles_rust::maze::maze::Maze;
use maze_tiles_rust::tile::tile::Tile;

let mut maze = Maze::new(6, 6);
maze.add_tile(Tile::new_path());
```