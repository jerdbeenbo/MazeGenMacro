use macroquad::prelude::*;

#[macroquad::main("Maze Generator")]
async fn main() {

    //set screen size
    let screen_width: f32 = 900.0;
    let screen_height: f32 = 900.0;
    request_new_screen_size(900.0, 900.0);

    //constants
    let columns = 10;
    let rows = 10;
    let column_size = (screen_width as i32) / columns;
    let row_size = (screen_height as i32) / rows;

    loop {
        clear_background(GRAY);

        //draw grid
        for i in 0..(columns as i32) { //NEEDS TO BE THE SAME AS COLUMNS  
            for x in 0..(rows as i32) { //NEEDS TO BE THE SAME AS ROWS
                draw_rectangle_lines((i*column_size) as f32, (x*row_size) as f32, column_size as f32, row_size as f32, 1.0, BLACK);
            }
        }
        //draw_text("Hello!", 20.0, 20.0, 30.0, RED);

        next_frame().await
    }
}
