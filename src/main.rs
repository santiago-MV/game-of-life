use macroquad:: prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut grid = vec![vec![0;1920];1080];

    grid[400][300] = 1;
    
    loop {
        clear_background(WHITE);
        
        

        draw(&grid);
        
        next_frame().await
    }
}

fn draw(grid:&Vec<Vec<i32>>){
    let mut row_index:f32 = 0.0;
    let mut col_index:f32 = 0.0;
    for row in grid {
        for column in row{
            if *column == 1{
                draw_rectangle(col_index, row_index, 10.0, 10.0, BLACK);
            }
            col_index += 1.0;
        }
        col_index = 0.0;
        row_index += 1.0;
    }
}
