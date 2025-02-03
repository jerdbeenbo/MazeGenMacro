use macroquad::prelude::*;

/*
    Upgrades:
    Have the generation run on multiple threads to speed up times
    Animate maze generation
    Wait for input for maze generation (different input for animated or not)

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


    //Generate maze (Recursive Backtracking)
    generate_maze(&mut cells, columns, rows);

    let mut show_maze: bool = false;

    loop {
        clear_background(GRAY);
        //draw_text("Hello!", 20.0, 20.0, 30.0, RED);

        //draw the maze
        if is_key_pressed(KeyCode::G) {
            show_maze = true;
        }

        if show_maze {
            draw_maze_unanimated(&cells, column_size, row_size); 
        }

        next_frame().await
    }
}

fn draw_maze_unanimated(cells: &Vec<Cell>, columns_size: i32, row_size: i32) {

    //loop through all cells
    for cell in cells {

        let x: f32 = cell.col_position as f32 * columns_size as f32;
        let y: f32 = cell.row_position as f32 * row_size as f32;

        //draw lines on active walls
        if cell.bottom_active {
            draw_line(x, y + row_size as f32, x + columns_size as f32, y + row_size as f32, 1.0, RED);
        }
        if cell.top_active {
            draw_line(x, y, x + columns_size as f32, y, 1.0, RED);
        }
        if cell.right_active {
            draw_line(x + columns_size as f32, y, x + columns_size as f32, y + row_size as f32, 1.0, RED);
        }
        if cell.left_active {
            draw_line(x, y, x, y + row_size as f32, 1.0, RED);
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

        if cells[l_index].visited == false {
            neighbours.push(l_index);
        }
    }
    if cells[index].col_position < columns - 1 {
        //right neighbour exists, store index information, check if univisited and save
        let r_index = index + 1;

        if cells[r_index].visited == false {
            neighbours.push(r_index);
        }
    }
    if cells[index].row_position > 0 {
        //top neighbour exists, store index information, check if univisited and save
        //let t_index = index - columns_as_usize;
        if let Some(t_index) = index.checked_sub(columns_as_usize) {
            if cells[t_index].visited == false {
                neighbours.push(t_index);
            }
        }
    }
    if cells[index].row_position < rows - 1 {
        //bottom neighbour exists, store index information, check if univisited and save
        //let b_index = index + columns_as_usize;
        if let Some(b_index) = index.checked_sub(columns_as_usize) {
            if cells[b_index].visited == false {
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
                
                //determine which wall is being removed for moving and remove walls
                remove_walls(current_neighbours[neighbour_chosen], current_cell_index, cells);

                //add current cell to backtracking stack
                backtracking_stack.push(current_cell_index);

                //set current cell to chosen neighbour
                current_cell_index = current_neighbours[neighbour_chosen];
            }
            else {
                //no viable neighbours, we need to backtrack

                if backtracking_stack.len() == 0 {
                    //is empty, generation is complete
                    generating = false;
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

        //remove walls
        cells[neighbour].left_active = false;
        cells[c_index].right_active = false;  
    }
    if cells[neighbour].col_position < cells[c_index].col_position {
        //we moved left

        //remove walls
        cells[neighbour].right_active = false;
        cells[c_index].left_active = false;  
    }
    if cells[neighbour].row_position > cells[c_index].row_position {
        //we moved down

        //remove walls
        cells[neighbour].top_active = false;
        cells[c_index].bottom_active = false;        
    }
    if cells[neighbour].row_position < cells[c_index].row_position {
        //we moved up

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

        return cells;
}
