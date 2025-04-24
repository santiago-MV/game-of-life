use macroquad::prelude::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut grid = vec![vec![0; GRID_WIDTH]; GRID_HEIGHT];

    let mut start_stop = false;
    let mut grid_show = true;
    loop {
        if is_key_pressed(KeyCode::Space) {
            start_stop = !start_stop;
        }
        if is_key_pressed(KeyCode::G) {
            grid_show = !grid_show;
        }
        if !start_stop && is_mouse_button_pressed(MouseButton::Left) {
            let height_scale = screen_height() / (GRID_HEIGHT as f32);
            let width_scale = screen_width() / (GRID_WIDTH as f32);
            let (column_pixel, row_pixel) = mouse_position();
            let row_index = (row_pixel / height_scale).floor() as usize;
            let column_index = (column_pixel / width_scale).floor() as usize;
            if grid[row_index][column_index] == 1 {
                grid[row_index][column_index] = 0;
            } else {
                grid[row_index][column_index] = 1;
            }
        }
        clear_background(WHITE);
        //Draw the current state of the grid
        draw(&grid, &grid_show);
        if start_stop {
            let neighbour_grid = calculate_neighbour_amount_grid(&grid);
            transition(&mut grid, &neighbour_grid);
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
        next_frame().await
    }
}
fn transition(grid: &mut Vec<Vec<i32>>, neighbours: &Vec<Vec<i32>>) {
    let mut row_index: usize = 0;
    let mut col_index: usize = 0;
    for row in neighbours {
        for col in row {
            if grid[row_index][col_index] == 1 {
                match *col {
                    0..=1 => grid[row_index][col_index] = 0,
                    2..=3 => grid[row_index][col_index] = 1,
                    _ => grid[row_index][col_index] = 0,
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
fn draw(grid: &Vec<Vec<i32>>, grid_show: &bool) {
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
            if *grid_show {
                draw_rectangle_lines(
                    col_index * width_scale,
                    row_index * height_scale,
                    width_scale,
                    height_scale,
                    1.0,
                    BLACK,
                );
            }
            col_index += 1.0;
        }
        col_index = 0.0;
        row_index += 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn underpopulation_test() {
        let mut test_grid = vec![vec![0; 100]; 100];
        //No neighbours
        test_grid[0][0] = 1;
        //One neighbour
        test_grid[3][3] = 1;
        test_grid[2][3] = 1;

        let neighbours = calculate_neighbour_amount_grid(&test_grid);
        transition(&mut test_grid, &neighbours);
        assert_eq!(test_grid, vec![vec![0; 100]; 100])
    }

    #[test]
    fn outlive() {
        let mut test_grid = vec![vec![0; 100]; 100];
        //Two neighbours
        test_grid[0][0] = 1;
        test_grid[0][1] = 1;
        test_grid[0][2] = 1;
        //Three neighbours
        test_grid[3][3] = 1;
        test_grid[2][2] = 1;
        test_grid[2][4] = 1;
        test_grid[4][3] = 1;

        let neighbours = calculate_neighbour_amount_grid(&test_grid);
        transition(&mut test_grid, &neighbours);
        assert_eq!(test_grid[0][1], 1);
        assert_eq!(test_grid[3][3], 1);
    }

    #[test]
    fn overpopulation() {
        let mut test_grid = vec![vec![0; 100]; 100];
        //Four neighbours
        test_grid[3][3] = 1;
        test_grid[2][2] = 1;
        test_grid[2][4] = 1;
        test_grid[2][3] = 1;
        test_grid[3][4] = 1;

        //>4 neighbours
        test_grid[6][6] = 1;
        test_grid[5][5] = 1;
        test_grid[5][6] = 1;
        test_grid[5][7] = 1;
        test_grid[6][5] = 1;
        test_grid[6][7] = 1;

        let neighbours = calculate_neighbour_amount_grid(&test_grid);
        transition(&mut test_grid, &neighbours);
        assert_eq!(test_grid[3][3], 0);
        assert_eq!(test_grid[6][6], 0);
    }

    #[test]
    fn reproduction() {
        let mut test_grid = vec![vec![0; 100]; 100];
        //Reproduction [3][3] has 3 neighbours
        test_grid[2][2] = 1;
        test_grid[2][4] = 1;
        test_grid[2][3] = 1;

        let neighbours = calculate_neighbour_amount_grid(&test_grid);
        transition(&mut test_grid, &neighbours);
        assert_eq!(test_grid[3][3], 1);
    }
}
