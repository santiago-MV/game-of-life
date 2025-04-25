# Game of life
An implementation of Conway's game of life using Rust and Macroquad
## Needed
macroquad = "0.4"

rust = "1.86.0"
## How to run it
1. Clone the github repo
2. Open the project folder ```cd game-of-life```
3. To build and run the project execute: ```make run```
## How to play
Each grid slot is a dead cell, click on cells to bring them alive!
When you're ready press space to start the game
To bring more cells alive the game must be paused, press space once again to pause it
Pressing G the visual grid can be toggled on or of
### Rules
Conway's game of life has 4 rules:
1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2. Any live cell with two or three live neighbours lives on to the next generation.
3. Any live cell with more than three live neighbours dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
