/*



*/

use std::f32::INFINITY;

use crate::Cell;



pub fn run_dijkstras(cells: &mut Vec<Cell>, columns: i32, rows: i32) {

    //start at position 0
    let mut current_cell_index: usize = 0;

    println!("{}",current_cell_index);

    //initialise the distance in cell 0 to 0
    cells[current_cell_index].distance_from_start = 0.0;

    let mut neighbour_infinity: bool = false;

    let mut solving: bool = true;
    while solving {
        
        //find unvisited cell (neighbour) with the smallest distance
        let shortest_neighbour_index: usize = get_shortest_neighbour(current_cell_index, cells, columns, rows);
        
        //check if the shortest distance found is STILL infinity AND there are no other settled cells to explore
        if cells[shortest_neighbour_index].distance_from_start == f32::INFINITY{
            //we are stuck, break the loop
            neighbour_infinity = true;
        }

        //have this complex logic locked behind a boolean check so it doesn't run every loop cycle, only if the algorithm is stuck
        if neighbour_infinity {
            let mut unusable_cells: usize = 0;
            //check if there are no more unsettled cells to explore
            for c in 0..cells.len() {
                //check each cells distance from start and settled status
                if cells[c].settled == true || cells[c].distance_from_start == f32::INFINITY{
                    unusable_cells = unusable_cells + 1;
                }
            }

            //check if there is anything left to explore
            if unusable_cells == cells.len() {
                //nothing left to explore
                solving = false;
            }
        }

        //mark current cell that we are in as settled (visited for dijkstras)
        cells[current_cell_index].settled = true;

        //move to the shortest neighbour
        current_cell_index = shortest_neighbour_index;

        if current_cell_index == &cells.len() - 1 {
            //reached the last cell (dijkstras solved)
            solving = false;
        }

    }

    //reconstruct the path back to the start
    let mut dijkstra_path: Vec<usize> = Vec::new();
    let length: &usize = &cells.len();
    current_cell_index = length - 1;

    //reconstruct starting at the end and working back to the start
    while current_cell_index != 0 {
        dijkstra_path.push(current_cell_index);
        current_cell_index = cells[current_cell_index].previous_cell_index.unwrap(); //unwrap because it is an option
    }

    //add start cell to the path
    dijkstra_path.push(0);

}

//returns INDEX of cells[] cell
fn get_shortest_neighbour(index: usize, cells: &mut Vec<Cell>, columns: i32, rows: i32) -> usize{

    let columns_as_usize: usize = columns as usize;

    //store neighbours
    let mut neighbours: Vec<usize> = Vec::new();
    //create index variables for storing
    //let (l_index, r_index, t_index, b_index): (usize, usize, usize, usize);

    //get all neighbours || declare what neighbours are possible
    if cells[index].col_position > 0 {
        //left neighbour exists, store index information, check if univisited and save
        let l_index = index - 1;

        if l_index < cells.len() && !cells[l_index].settled {

            //check if right wall for this cell is open or closed
            if cells[l_index].right_active == false {
                //wall is not there, save as a potential neighbour
                neighbours.push(l_index);
            }
        }
    }
    if cells[index].col_position < columns - 1 {
        //right neighbour exists, store index information, check if univisited and save
        let r_index = index + 1;

        if r_index < cells.len() && !cells[r_index].settled {

            //check if left wall for this cell is open or closed
            if cells[r_index].left_active == false {
                //wall is not there, save as a potential neighbour
                neighbours.push(r_index);
            }
        }
    }
    if cells[index].row_position > 0 {
        //top neighbour exists, store index information, check if univisited and save
        //let t_index = index - columns_as_usize;
        //try statement to prevent overflow panic, if overflow would have occured neighbour doesnt exist and skip
        if let Some(t_index) = index.checked_sub(columns_as_usize) {

            if t_index < cells.len() && !cells[t_index].settled {
                
                //check if bottom wall for this cell is open or closed
                if cells[t_index].bottom_active == false {
                    //wall is not there, save as a potential neighbour
                    neighbours.push(t_index);
                }
            }
        }
    }
    if cells[index].row_position < rows - 1 {
        //bottom neighbour exists, store index information, check if univisited and save
        //let b_index = index + columns_as_usize;
        //try statement to prevent overflow panic, if overflow would have occured neighbour doesnt exist and skip
        if let Some(b_index) = index.checked_add(columns_as_usize) {
                       
            if b_index < cells.len() && !cells[b_index].settled {
                
                //check if top wall for this cell is open or closed
                if cells[b_index].top_active == false {
                    //wall is not there, save as a potential neighbour
                    neighbours.push(b_index);
                }
            }
        }
    }

    
    //determine which neighbour is the shortest from list of neighbours
    let mut shortest_distance = f32::INFINITY;
    let mut shortest_index = 0;
    
    for &neighbor_index in &neighbours {
        let potential_distance = cells[index].distance_from_start + 1.0;
        
        // If we found a shorter path to this neighbor
        if potential_distance < cells[neighbor_index].distance_from_start {
            // We would update the distance here, but Cell needs to be mutable
            cells[neighbor_index].distance_from_start = potential_distance;
            cells[neighbor_index].previous_cell_index = Some(index);
        }
        
        // Keep track of neighbor with shortest distance
        if cells[neighbor_index].distance_from_start < shortest_distance {
            shortest_distance = cells[neighbor_index].distance_from_start;
            shortest_index = neighbor_index;
        }
    }
    
    return shortest_index;

}