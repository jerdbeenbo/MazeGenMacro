/*



*/

use crate::Cell;



pub fn run_dijkstras(cells: &Vec<Cell>, columns: i32, rows: i32) {

    //start at position 0
    let current_cell_index: usize = 0;

    let solving: bool = true;
    while solving {
        
        //find unvisited cell (neighbour) with the smallest distance
        let shortest_neighbour_index: usize = get_shortest_neighbour(current_cell_index, &cells, columns, rows);
    }

}

//returns INDEX of cells[] cell
fn get_shortest_neighbour(index: usize, cells: &Vec<Cell>, columns: i32, rows: i32) -> usize{

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

    for n in &neighbours {
        println!("{}", n);
    }

    //return
    return neighbours[0];

}