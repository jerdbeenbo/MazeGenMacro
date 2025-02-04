use macroquad::prelude::*;

/*
    Upgrades:
    Have the generation run on multiple threads to speed up times
    Animate maze generation
    Wait for input for maze generation (different input for animated or not)
    Add a "completion line" option after maze is generated -> solve_maze()

*/

//Cell structure to represent each cell in the grid
struct Cell {
    //Cells position in the grid (row, column)
    row_position: i32,
    col_position: i32,

    //whether or not its walls are active (closed or open)
    top_active: bool,
    bottom_active: bool,
    left_active: bool,
    right_active: bool,

    //has the maze generation algorithm travelled to this cell yet?
    visited: bool
}

//Structure for holding our state (used for animation)
struct MazeState {
    visited_cell_index: Vec<usize>,
    backtracking_stack: Vec<usize>,
    current_cell_index: usize,
    generating: bool
}

#[macroquad::main("Maze Generator")]
async fn main() {

    //set screen size
    let screen_width: f32 = 800.0;
    let screen_height: f32 = 800.0;
    request_new_screen_size(800.0, 830.0);

    //constants
    let columns = 40;
    let rows = 40; // -> Columns and rows should be cleany dividable into screen width/height
    let column_size = (screen_width as i32) / columns;
    let row_size = (screen_height as i32) / rows;

    //set up cells
    let mut cells: Vec<Cell> = cell_setup(columns, rows);

    //initialise maze state
    let mut maze_state: MazeState = initialise_maze_state();


    //Generate maze (Recursive Backtracking) /*this will be unanimated */
    //generate_maze(&mut cells, columns, rows);

    let mut show_maze: bool = false;

    loop {
        clear_background(GRAY);
    
        if is_key_pressed(KeyCode::G) {
            show_maze = true;
        }
    
        if show_maze {
            // Draw the current state of the maze
            draw_maze_unanimated(&cells, column_size, row_size);
    
            // If still generating, do one step
            if maze_state.generating {
                generate_maze_step(&mut cells, &mut maze_state, columns, rows);
            }
        }
    
        next_frame().await
    }
}

fn generate_maze_step(cells: &mut Vec<Cell>, state: &mut MazeState, columns: i32, rows: i32) {
    
    //check if we are still generating
    if !state.generating {
        return;
    }
    else {
        // Check if complete
        if cells.len() == state.visited_cell_index.len() {
            state.generating = false;
            return;
        }
    
        // Get unvisited neighbors
        let current_neighbours = get_unvisited_neighbours(state.current_cell_index, cells, columns, rows);
    
        if current_neighbours.len() > 0 {
            // Mark current cell as visited and store cell index in visited stack
            cells[state.current_cell_index].visited = true;
            state.visited_cell_index.push(state.current_cell_index);
        
            // Pick random neighbor to move to
            let neighbour_chosen = rand::gen_range(0, current_neighbours.len());
            let next_cell = current_neighbours[neighbour_chosen];
            cells[current_neighbours[neighbour_chosen]].visited = true;
        
            // Remove walls between cells
            remove_walls(current_neighbours[neighbour_chosen], state.current_cell_index, cells);
        
            // Add current to backtracking stack
            state.backtracking_stack.push(state.current_cell_index);
        
            // Move to new cell
            state.current_cell_index = next_cell;
        } 
        else {
            // No neighbors, need to backtrack
            if state.backtracking_stack.len() == 0 {
                if state.visited_cell_index.len() == cells.len() {
                    state.generating = false;
                } 
                else {
                    // Handle unvisited cells like before
                    state.current_cell_index = cells.len() - 1;
                    cells[state.current_cell_index].visited = true;
                    state.visited_cell_index.push(state.current_cell_index);
                    
                    // Connect to visited neighbor
                    let left_neighbor = state.current_cell_index - 1;
                    if cells[left_neighbor].visited {
                        remove_walls(state.current_cell_index, left_neighbor, cells);
                    }
                }
            } 
            else {
                state.current_cell_index = state.backtracking_stack.pop().unwrap();
            }
        }
    }
}

fn draw_maze_unanimated(cells: &Vec<Cell>, columns_size: i32, row_size: i32) {

    let line_colour: Color = BLACK;

    //loop through all cells
    for cell in cells {

        let x: f32 = cell.col_position as f32 * columns_size as f32;
        let y: f32 = cell.row_position as f32 * row_size as f32;

        //draw lines on active walls
        if cell.bottom_active {
            draw_line(x, y + row_size as f32, x + columns_size as f32, y + row_size as f32, 3.0, line_colour);
        }
        if cell.top_active {
            draw_line(x, y, x + columns_size as f32, y, 3.0, line_colour);
        }
        if cell.right_active {
            draw_line(x + columns_size as f32, y, x + columns_size as f32, y + row_size as f32, 3.0, line_colour);
        }
        if cell.left_active {
            draw_line(x, y, x, y + row_size as f32, 3.0, line_colour);
        }
    }
}

fn get_unvisited_neighbours(index: usize, cells: &mut Vec<Cell>, columns: i32, rows: i32) -> Vec<usize>{

    let columns_as_usize: usize = columns as usize;

    //store neighbours
    let mut neighbours: Vec<usize> = Vec::new();
    //create index variables for storing
    //let (l_index, r_index, t_index, b_index): (usize, usize, usize, usize);

    //get all neighbours || declare what neighbours are possible
    if cells[index].col_position > 0 {
        //left neighbour exists, store index information, check if univisited and save
        let l_index = index - 1;

        if l_index < cells.len() && !cells[l_index].visited {
            neighbours.push(l_index);
        }
    }
    if cells[index].col_position < columns - 1 {
        //right neighbour exists, store index information, check if univisited and save
        let r_index = index + 1;

        if r_index < cells.len() && !cells[r_index].visited {
            neighbours.push(r_index);
        }
    }
    if cells[index].row_position > 0 {
        //top neighbour exists, store index information, check if univisited and save
        //let t_index = index - columns_as_usize;
        //try statement to prevent overflow panic, if overflow would have occured neighbour doesnt exist and skip
        if let Some(t_index) = index.checked_sub(columns_as_usize) {

            if t_index < cells.len() && !cells[t_index].visited {
                neighbours.push(t_index);
            }
        }
    }
    if cells[index].row_position < rows - 1 {
        //bottom neighbour exists, store index information, check if univisited and save
        //let b_index = index + columns_as_usize;
        //try statement to prevent overflow panic, if overflow would have occured neighbour doesnt exist and skip
        if let Some(b_index) = index.checked_add(columns_as_usize) {
                       
            if b_index < cells.len() && !cells[b_index].visited {
                neighbours.push(b_index);
            }
        }
    }

    return neighbours;
}

fn generate_maze(cells: &mut Vec<Cell>, c: i32, r: i32) {

    //stack to keep track of visited cells
    let mut visited_cell_index: Vec<usize> = Vec::new();

    //backtracking stack for when hitting a dead end
    let mut backtracking_stack: Vec<usize> = Vec::new();
    
    //start at cell 0
    let mut current_cell_index = 0;

    let mut generating: bool = true;

    //iterate over all the avaliable cells starting at cell 0 (top left corner)
    while generating {
        //check if this cell has been visited
        if cells.len() == visited_cell_index.len() {
            
            // Before ending generation, check last cell
            let last_cell = cells.len() - 1;

            if cells[last_cell].left_active && cells[last_cell].top_active {
                // Last cell is isolated, try to connect it
                if cells[last_cell].col_position > 0 {
                    // Connect to left neighbor
                    remove_walls(last_cell, last_cell - 1, cells);
                } 
                else if cells[last_cell].row_position > 0 {
                    // Connect to top neighbor
                    remove_walls(last_cell, last_cell - c as usize, cells);
                }
            }
            //generation complete
            generating = false;
        }
        else {        
            //get current neighbours of the cell
            let current_neighbours: Vec<usize> = get_unvisited_neighbours(current_cell_index, cells, c, r);

            //check if there are any viable neighbours recorded
            if current_neighbours.len() > 0 {
                //we have potential neighbours

                //mark current cell as visited and store cell index in visted stack
                cells[current_cell_index].visited = true;
                visited_cell_index.push(current_cell_index);

                //pick a random neighbour to move to
                let neighbour_chosen: usize = rand::gen_range(0, current_neighbours.len());
                let next_cell = current_neighbours[neighbour_chosen];
                cells[current_neighbours[neighbour_chosen]].visited = true;

                //determine which wall is being removed for moving and remove walls
                remove_walls(current_neighbours[neighbour_chosen], current_cell_index, cells);

                //add current cell to backtracking stack
                backtracking_stack.push(current_cell_index);

                //set current cell to chosen neighbour
                current_cell_index = next_cell;
            }
            else {
                //no viable neighbours, we need to backtrack           

                if backtracking_stack.len() == 0 {
                    if visited_cell_index.len() == cells.len() {
                        generating = false;
                    } 
                    else {
                        // Move to last unvisited cell directly
                        current_cell_index = cells.len() - 1;
                        cells[current_cell_index].visited = true;  // Mark it visited!
                        visited_cell_index.push(current_cell_index);
                        
                        // Connect to one of its neighbors (prefer left)
                        let left_neighbor = current_cell_index - 1;
                        if cells[left_neighbor].visited {
                            remove_walls(current_cell_index, left_neighbor, cells);
                        } 
                        else {
                            let top_neighbor = current_cell_index - c as usize;
                            if cells[top_neighbor].visited {
                                remove_walls(current_cell_index, top_neighbor, cells);
                            }
                        }
                    }
                }
                else {
                    //save cell index to last position
                    current_cell_index = backtracking_stack.pop().unwrap();                    
                }
            }

        }
    }
}

fn remove_walls(neighbour: usize, c_index: usize, cells: &mut Vec<Cell>) {

    //determine which direction we moved
    if cells[neighbour].col_position > cells[c_index].col_position {
        //we moved right

        //println!("Moving right from {} to {}", c_index, neighbour);

        //remove walls
        cells[neighbour].left_active = false;
        cells[c_index].right_active = false;  
    }
    if cells[neighbour].col_position < cells[c_index].col_position {
        //we moved left

        //println!("Moving left from {} to {}", c_index, neighbour);

        //remove walls
        cells[neighbour].right_active = false;
        cells[c_index].left_active = false;  
    }
    if cells[neighbour].row_position > cells[c_index].row_position {
        //we moved down

        //println!("Moving down from {} to {}", c_index, neighbour);

        //remove walls
        cells[neighbour].top_active = false;
        cells[c_index].bottom_active = false;        
    }
    if cells[neighbour].row_position < cells[c_index].row_position {
        //we moved up

        //println!("Moving up from {} to {}", c_index, neighbour);

        //remove walls
        cells[neighbour].bottom_active = false;
        cells[c_index].top_active = false;
    }

}

fn cell_setup(c: i32, r: i32) -> Vec<Cell>{
    //vector to store all the cells
    let mut cells: Vec<Cell> = Vec::new();

        //cell setup
        for col in 0..(c as i32) { //NEEDS TO BE THE SAME AS COLUMNS  
            for row in 0..(r as i32) { //NEEDS TO BE THE SAME AS ROWS
                //draw_rectangle_lines((i*column_size) as f32, (x*row_size) as f32, column_size as f32, row_size as f32, 1.0, BLACK);
                cells.push(Cell { 
                    row_position: row, 
                    col_position: col, 
                    top_active: true, 
                    bottom_active: true, 
                    left_active: true, 
                    right_active: true, 
                    visited: false // -> Initialise with a full "grid" with all walls active and fully unvisited
                });
            }
        }

        // Remove top wall of first cell (entrance)
        cells[0].top_active = false;

        // Remove bottom wall of last cell (exit)
        let last_cell = cells.len() - 1;
        cells[last_cell].bottom_active = false;

        return cells;
}

fn initialise_maze_state() -> MazeState {

    let mut maze_state = MazeState {
        visited_cell_index: Vec::new(),
        backtracking_stack: Vec::new(),
        current_cell_index: 0,
        generating: true
    };

    return maze_state;
}