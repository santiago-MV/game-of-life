use macroquad::prelude::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut grid = vec![vec![0; GRID_WIDTH]; GRID_HEIGHT];
    grid[50][50] = 1;
    grid[51][50] = 1;
    grid[49][50] = 1;
    grid[50][49] = 1;
    grid[51][51] = 1;
    loop {
        clear_background(WHITE);
        //Draw the current state of the grid
        draw(&grid);

        let neighbour_grid = calculate_neighbour_amount_grid(&grid);
        let mut row_index: usize = 0;
        let mut col_index: usize = 0;
        for row in &neighbour_grid {
            for col in row {
                if grid[row_index][col_index] == 1 {
                    match *col {
                        0..=1 => grid[row_index][col_index] = 0,
                        4..=8 => grid[row_index][col_index] = 0,
                        _ => grid[row_index][col_index] = 1,
                    }
                } else {
                    if *col == 3 {
                        grid[row_index][col_index] = 1;
                    }
                }
                col_index += 1;
            }
            col_index = 0;
            row_index += 1;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
        next_frame().await
    }
}
//Calculate neighbours
fn calculate_neighbour_amount_grid(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut neighbour_amount_grid = vec![vec![0; GRID_WIDTH]; GRID_HEIGHT];
    let mut row_index: usize = 0;
    let mut column_index: usize = 0;
    const MAX_ROW: usize = GRID_HEIGHT - 1;
    const MAX_COL: usize = GRID_WIDTH - 1;
    //Iterate through the rows of the grid
    for row in grid {
        //Iterate through the columns of the row
        for _ in row {
            //Calculate the amounts of neighbours a cell has
            let mut neighbour_amount = 0;
            //ADD neighbours UP and DOWN
            match row_index {
                0 => neighbour_amount += grid[row_index + 1][column_index], //ADD only DOWN
                MAX_ROW => neighbour_amount += grid[row_index - 1][column_index], //ADD only UP
                _ => {
                    neighbour_amount += grid[row_index+1][column_index]  //ADD both
                                            + grid[row_index-1][column_index]
                }
            }
            //ADD neighbours LEFT and RIGHT
            match column_index {
                0 => neighbour_amount += grid[row_index][column_index + 1], //ADD only right
                MAX_COL => neighbour_amount += grid[row_index][column_index - 1], //ADD only left
                _ => {
                    neighbour_amount += grid[row_index][column_index+1]  //ADD both
                                            + grid[row_index][column_index-1]
                }
            }
            //ADD neighbours diagonally
            match (row_index, column_index) {
                (0, 0) => neighbour_amount += grid[row_index + 1][column_index + 1], //ADD DOWN RIGHT only
                (0, MAX_COL) => neighbour_amount += grid[row_index + 1][column_index - 1], //ADD DOWN LEFT only
                (MAX_ROW, 0) => neighbour_amount += grid[row_index - 1][column_index + 1], //ADD UP RIGHT only
                (MAX_ROW, MAX_COL) => neighbour_amount += grid[row_index - 1][column_index - 1], //ADD UP LEFT only
                (0, _) => {
                    neighbour_amount += grid[row_index+1][column_index+1]    //ADD both DOWN RIGHT and LEFT
                                                        +  grid[row_index+1][column_index-1]
                }
                (MAX_ROW, _) => {
                    neighbour_amount += grid[row_index-1][column_index+1]    //ADD both UP RIGHT and LEFT
                                                        +  grid[row_index-1][column_index-1]
                }
                (_, 0) => {
                    neighbour_amount += grid[row_index+1][column_index+1]    //ADD both RIGHT UP and DOWN
                                                        +  grid[row_index-1][column_index+1]
                }
                (_, MAX_COL) => {
                    neighbour_amount += grid[row_index+1][column_index-1]    //ADD both LEFT UP and DOWN
                                                        +  grid[row_index-1][column_index-1]
                }
                (_, _) => {
                    neighbour_amount += grid[row_index+1][column_index+1]    //ADD ALL
                                                        + grid[row_index+1][column_index-1]
                                                        + grid[row_index-1][column_index+1]
                                                        + grid[row_index-1][column_index-1]
                }
            }
            //Assign the calculated value to that position in the grid
            neighbour_amount_grid[row_index][column_index] = neighbour_amount;
            //Move onto the next column
            column_index += 1;
        }
        //Reset column index and move onto the next row
        column_index = 0;
        row_index += 1;
    }
    neighbour_amount_grid
}
//Draws the grid as squares
fn draw(grid: &Vec<Vec<i32>>) {
    let height_scale = screen_height() / GRID_HEIGHT as f32;
    let width_scale = screen_width() / GRID_WIDTH as f32;
    let mut row_index: f32 = 0.0;
    let mut col_index: f32 = 0.0;
    for row in grid {
        for column in row {
            if *column == 1 {
                draw_rectangle(
                    col_index * width_scale,
                    row_index * height_scale,
                    width_scale,
                    height_scale,
                    BLACK,
                );
            }
            col_index += 1.0;
        }
        col_index = 0.0;
        row_index += 1.0;
    }
}
