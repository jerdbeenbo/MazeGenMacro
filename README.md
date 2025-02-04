# Maze Generator

A recursive backtracking maze generator built with Rust and Macroquad. Generates and visualizes mazes in real-time.

## Features

- Generates mazes using recursive backtracking algorithm
- Real-time visualization of maze generation process
- Customizable grid size (default 40x40)
- Entry and exit points automatically created
- Animated generation process

## Controls

- Press `G` to show the generated maze and start generation
- Watch as the maze builds itself cell by cell

## Technical Details

### Algorithm

Uses a recursive backtracking algorithm which:
1. Starts at a random cell
2. Marks current cell as visited
3. Gets unvisited neighbors
4. Randomly chooses a neighbor
5. Removes walls between current cell and chosen neighbor
6. Repeats until all cells are visited
7. Backtracks when no unvisited neighbors are available

### Structure

- `Cell`: Struct representing each cell in the maze
  - Tracks position (row, column)
  - Tracks wall states (top, bottom, left, right)
  - Tracks visited state

- `MazeState`: Struct handling generation state
  - Tracks visited cells
  - Manages backtracking stack
  - Controls generation process

## Future Improvements

- [ ] Multi-threaded generation for improved performance
- [ ] Maze solving visualization
- [ ] Different maze generation algorithms
- [ ] User controls for animation speed
- [ ] Custom grid size input

## Requirements

- Rust
- Macroquad

## Building and Running

1. Clone the repository
2. Run with:
```bash
cargo run


## Read me generated with AI