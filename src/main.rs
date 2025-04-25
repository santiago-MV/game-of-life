use macroquad::prelude::*;
//SET THE GRID VALUES
const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut matrix = vec![vec![0; GRID_WIDTH]; GRID_HEIGHT];

    let mut is_running = false;
    let mut is_grid_showing = false;
    loop {
        if is_key_pressed(KeyCode::Space) {
            is_running = !is_running;
        }
        let status: &str;
        if is_running {
            status = "RUNNING"
        } else {
            status = "PAUSED"
        }
        if is_key_pressed(KeyCode::G) {
            is_grid_showing = !is_grid_showing;
        }
        if !is_running && is_mouse_button_pressed(MouseButton::Left) {
            process_clicks(&mut matrix);
        }
        //Color background
        clear_background(WHITE);
        //Draw the current state of the grid
        draw(&matrix, &is_grid_showing);
        //DRAW text
        draw_text(status, 5.0, 30.0, 50.0, DARKBLUE);
        draw_text(
            "Cells can be edited on pause",
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
        //Do transitions
        if is_running {
            //Calculate neighbour_grid and transitions
            let neighbour_grid = calculate_neighbour_amount_matrix(&matrix);
            transition(&mut matrix, &neighbour_grid);
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
        next_frame().await
    }
}
fn transition(matrix: &mut Vec<Vec<i32>>, neighbours: &Vec<Vec<i32>>) {
    let mut row_index: usize = 0;
    let mut col_index: usize = 0;
    for row in neighbours {
        for col in row {
            if matrix[row_index][col_index] == 1 {
                match *col {
                    0..=1 => matrix[row_index][col_index] = 0,
                    2..=3 => matrix[row_index][col_index] = 1,
                    _ => matrix[row_index][col_index] = 0,
                }
            } else {
                if *col == 3 {
                    matrix[row_index][col_index] = 1;
                }
            }
            col_index += 1;
        }
        col_index = 0;
        row_index += 1;
    }
}
//Process clicks
fn process_clicks(matrix: &mut Vec<Vec<i32>>) {
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
//Calculate neighbours
fn calculate_neighbour_amount_matrix(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut neighbour_amount_matrix = vec![vec![0; matrix[0].len()]; matrix.len()];
    let mut row_index: usize = 0;
    let mut column_index: usize = 0;
    //Iterate through the rows of the grid
    for row in matrix {
        //Iterate through the columns of the row
        for _ in row {
            //Calculate the amounts of neighbours a cell has and assing it in the grid
            neighbour_amount_matrix[row_index][column_index] =
                get_neighbours_for(row_index, column_index, matrix);
            //Move onto the next column
            column_index += 1;
        }
        //Reset column index and move onto the next row
        column_index = 0;
        row_index += 1;
    }
    neighbour_amount_matrix
}
//Get neighbours
fn get_neighbours_for(row_index: usize, col_index: usize, matrix: &Vec<Vec<i32>>) -> i32 {
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
//Draws the matrix as squares
fn draw(matrix: &Vec<Vec<i32>>, grid_show: &bool) {
    let height_scale = screen_height() / matrix.len() as f32;
    let width_scale = screen_width() / matrix[0].len() as f32;

    let mut row_index: f32 = 0.0;
    let mut col_index: f32 = 0.0;
    for row in matrix {
        if *grid_show {
            draw_line(
                0.0,
                row_index * height_scale,
                screen_width(),
                row_index * height_scale,
                1.0,
                LIGHTGRAY,
            );
        }
        for column in row {
            if *grid_show {
                draw_line(
                    col_index * width_scale,
                    0.0,
                    col_index * width_scale,
                    screen_height(),
                    1.0,
                    LIGHTGRAY,
                );
            }
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

        let neighbours = calculate_neighbour_amount_matrix(&test_matrix);
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

        let neighbours = calculate_neighbour_amount_matrix(&test_matrix);
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

        let neighbours = calculate_neighbour_amount_matrix(&test_matrix);
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

        let neighbours = calculate_neighbour_amount_matrix(&test_matrix);
        transition(&mut test_matrix, &neighbours);
        assert_eq!(test_matrix[3][3], 1);
    }
}
