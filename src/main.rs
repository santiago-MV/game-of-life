use macroquad::prelude::*;
//Set the matrix dimensions
//Amount of rows (height) and columns (width)
const MATRIX_WIDTH: usize = 100;
const MATRIX_HEIGHT: usize = 100;

#[macroquad::main("BasicShapes")]
async fn main() {
    //Matrix that represents the current state of the game
    //Each item represents a cell whos values can be:
    //1 -> Live cell
    //0 -> Death cell
    let mut matrix = vec![vec![0; MATRIX_WIDTH]; MATRIX_HEIGHT];

    let mut is_running = false;
    let mut is_grid_showing = false;
    loop {
        if is_key_pressed(KeyCode::Space) {
            is_running = !is_running;
        }
        let status: &str = if is_running { "RUNNING" } else { "PAUSED" };
        if is_key_pressed(KeyCode::G) {
            is_grid_showing = !is_grid_showing;
        }
        if !is_running && is_mouse_button_pressed(MouseButton::Left) {
            process_clicks(&mut matrix);
        }
        clear_background(WHITE);
        draw(&matrix, &is_grid_showing);
        draw_text(status, 5.0, 30.0, 50.0, DARKBLUE);
        draw_text(
            "Cells can only be edited on pause",
            5.0,
            screen_height() - 50.0,
            25.0,
            DARKBLUE,
        );
        draw_text(
            "Press <space> to start/pause the game",
            5.0,
            screen_height() - 30.0,
            25.0,
            DARKBLUE,
        );
        draw_text(
            "Press <G> to toggle grid on/off",
            5.0,
            screen_height() - 10.0,
            25.0,
            DARKBLUE,
        );
        //Calculate the next state of the game -> That means which cells live through and which doesn't
        if is_running {
            //Get the matrix with the amount of live neighbours for each cell
            let alice_neighbours_matrix = calculate_alive_neighbour_amount_matrix(&matrix);
            //Update cells state, define which cells pass to the next generation
            transition(&mut matrix, &alice_neighbours_matrix);
        }
        std::thread::sleep(std::time::Duration::from_millis(150));
        next_frame().await
    }
}
//Calculate the next state of each cell using their current state and number of alive neighbours
//The matrix arugment represents the current state of each cell
//The alive_neighbours_amount_matrix argument represents the amount of live neighbours each cell has
fn transition(matrix: &mut [Vec<i32>], alive_neighbours_amount_matrix: &[Vec<i32>]) {
    for (row_index, row) in alive_neighbours_amount_matrix.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if matrix[row_index][col_index] == 1 {
                match *col {
                    0..=1 => matrix[row_index][col_index] = 0,
                    2..=3 => matrix[row_index][col_index] = 1,
                    _ => matrix[row_index][col_index] = 0,
                }
            } else if *col == 3 {
                matrix[row_index][col_index] = 1;
            }
        }
    }
}
//Logic for bringing a cell alive after it was clicked by the user
//The matrix argument represents the current state, the state of the clicked cell will be updated in that matrix
fn process_clicks(matrix: &mut [Vec<i32>]) {
    let height_scale = screen_height() / (matrix.len() as f32);
    let width_scale = screen_width() / (matrix[0].len() as f32);
    let (column_pixel, row_pixel) = mouse_position();
    let row_index = (row_pixel / height_scale).floor() as usize;
    let column_index = (column_pixel / width_scale).floor() as usize;
    if matrix[row_index][column_index] == 1 {
        matrix[row_index][column_index] = 0;
    } else {
        matrix[row_index][column_index] = 1;
    }
}
//Calculate a matrix with the amount of alive neighbours each cell has
//Each cells neighbour are the other cells that are horizontally, vertically or diagonally adjacent to it (8 max)
//The argument represents the current matrix state
//The argument matrix and the one returned share dimensions, so indexes represent the same cell in both matrices
//The returned matrix has the amount of alive neighbours for each cell
fn calculate_alive_neighbour_amount_matrix(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut neighbour_amount_matrix = vec![vec![0; matrix[0].len()]; matrix.len()];
    //Iterate through the rows of the grid
    for (row_index, row) in matrix.iter().enumerate() {
        //Iterate through the columns of the row
        for (column_index, _) in row.iter().enumerate() {
            //Calculate the amounts of neighbours a cell has and assing it in the grid
            neighbour_amount_matrix[row_index][column_index] =
                get_amount_of_alive_neighbours(row_index, column_index, matrix);
        }
    }
    neighbour_amount_matrix
}
//Get the amount of alive neighbours a cell has
//The matrix represents the current state of the game
//The first two arguments represent the indexes of that cell in the matrix (cell position)
//The returned value represents the amount of alive neighbours the cell has
fn get_amount_of_alive_neighbours(row_index: usize, col_index: usize, matrix: &[Vec<i32>]) -> i32 {
    let cell_row = row_index as i32;
    let cell_col = col_index as i32;
    //Init an array with all the coordinates of the cell neighbours
    let neighbour_coordinates = [
        (cell_row - 1, cell_col - 1), //UP Left
        (cell_row - 1, cell_col),     //UP
        (cell_row - 1, cell_col + 1), //UP Right
        (cell_row, cell_col - 1),     //Left
        (cell_row, cell_col + 1),     //Right
        (cell_row + 1, cell_col - 1), //DOWN left
        (cell_row + 1, cell_col),     //DOWN
        (cell_row + 1, cell_col + 1),
    ]; //DOWN Right
    //Filter out neighbours that are out of bounds
    let neighbours_on_matrix: Vec<&(i32, i32)> = neighbour_coordinates
        .iter()
        .filter(|(row, col)| {
            (0..(matrix.len() as i32)).contains(row) && (0..(matrix[0].len() as i32)).contains(col)
        })
        .collect();
    //Filter death cells
    let live_neighbours: Vec<_> = neighbours_on_matrix
        .iter()
        .filter(|(row, col)| matrix[*row as usize][*col as usize] == 1)
        .collect();
    //Return the amount of live neighbours
    live_neighbours.len() as i32
}
//Draw the current state of the game
//Each cell is represented by a rectangle that can be:
//White -> Dead cell
//Black -> Alive cell
fn draw(matrix: &[Vec<i32>], grid_show: &bool) {
    let height_scale = screen_height() / matrix.len() as f32;
    let width_scale = screen_width() / matrix[0].len() as f32;

    for (row_index, row) in matrix.iter().enumerate() {
        if *grid_show {
            //Draws the horizontal lines of the grid
            draw_line(
                0.0,
                (row_index as f32) * height_scale,
                screen_width(),
                (row_index as f32) * height_scale,
                1.0,
                LIGHTGRAY,
            );
        }
        for (col_index, column) in row.iter().enumerate() {
            if *grid_show {
                //Draws the vertical lines of the grid
                draw_line(
                    (col_index as f32) * width_scale,
                    0.0,
                    (col_index as f32) * width_scale,
                    screen_height(),
                    1.0,
                    LIGHTGRAY,
                );
            }
            if *column == 1 {
                //Draw cells
                draw_rectangle(
                    (col_index as f32) * width_scale,
                    (row_index as f32) * height_scale,
                    width_scale,
                    height_scale,
                    BLACK,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn underpopulation_test() {
        let mut test_matrix = vec![vec![0; 4]; 4];
        //No neighbours
        test_matrix[0][0] = 1;
        //One neighbour
        test_matrix[3][3] = 1;
        test_matrix[2][3] = 1;

        let neighbours = calculate_alive_neighbour_amount_matrix(&test_matrix);
        transition(&mut test_matrix, &neighbours);
        assert_eq!(test_matrix, vec![vec![0; 4]; 4])
    }

    #[test]
    fn outlive() {
        let mut test_matrix = vec![vec![0; 5]; 5];
        //Two neighbours
        test_matrix[0][0] = 1;
        test_matrix[0][1] = 1;
        test_matrix[0][2] = 1;
        //Three neighbours
        test_matrix[3][3] = 1;
        test_matrix[2][2] = 1;
        test_matrix[2][4] = 1;
        test_matrix[4][3] = 1;

        let neighbours = calculate_alive_neighbour_amount_matrix(&test_matrix);
        transition(&mut test_matrix, &neighbours);
        assert_eq!(test_matrix[0][1], 1);
        assert_eq!(test_matrix[3][3], 1);
    }

    #[test]
    fn overpopulation() {
        let mut test_matrix = vec![vec![0; 10]; 10];
        //Four neighbours
        test_matrix[3][3] = 1;
        test_matrix[2][2] = 1;
        test_matrix[2][4] = 1;
        test_matrix[2][3] = 1;
        test_matrix[3][4] = 1;

        //>4 neighbours
        test_matrix[6][6] = 1;
        test_matrix[5][5] = 1;
        test_matrix[5][6] = 1;
        test_matrix[5][7] = 1;
        test_matrix[6][5] = 1;
        test_matrix[6][7] = 1;

        let neighbours = calculate_alive_neighbour_amount_matrix(&test_matrix);
        transition(&mut test_matrix, &neighbours);
        assert_eq!(test_matrix[3][3], 0);
        assert_eq!(test_matrix[6][6], 0);
    }

    #[test]
    fn reproduction() {
        let mut test_matrix = vec![vec![0; 5]; 5];
        //Reproduction [3][3] has 3 neighbours
        test_matrix[2][2] = 1;
        test_matrix[2][4] = 1;
        test_matrix[2][3] = 1;

        let neighbours = calculate_alive_neighbour_amount_matrix(&test_matrix);
        transition(&mut test_matrix, &neighbours);
        assert_eq!(test_matrix[3][3], 1);
    }
}
