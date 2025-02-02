use std::thread::current;

use macroquad::prelude::*;

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
    generate_maze(&mut cells);

    loop {
        clear_background(GRAY);
        //draw_text("Hello!", 20.0, 20.0, 30.0, RED);

        next_frame().await
    }
}

fn generate_maze(cells: &mut Vec<Cell>) {

    //stack to keep track of visited cells
    let visited: Vec<Cell> = Vec::new();
    
    
    //start at cell 0
    let mut current_cell_index = 0;

    let generating: bool = true;

    //iterate over all the avaliable cells starting at cell 0 (top left corner)
    while generating {
        //check if this cell has been visited
        if cells.len() == visited.len() {
            //generation complete

        }
        else {
            //there are still unvisited cells
            //get unvisited neighbours
            //randomly choose one neighbour
            //remove walls between current and chosen cell
            //make the chosen cell the new current cell
            //add current cell to visited stack

            //add current cell to visited
            
            //mark current cell as visited
            cells[current_cell_index].visited = true;
            visited.push(cells[current_cell_index]);
        }
    }
}

fn cell_setup(c: i32, r: i32) -> Vec<Cell>{
    //vector to store all the cells
    let mut cells: Vec<Cell> = Vec::new();

        //cell setup
        for col in 0..(c as i32) { //NEEDS TO BE THE SAME AS COLUMNS  
            for row in 0..(r as i32) { //NEEDS TO BE THE SAME AS ROWS
                //draw_rectangle_lines((i*column_size) as f32, (x*row_size) as f32, column_size as f32, row_size as f32, 1.0, BLACK);
                println!("Working");
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
