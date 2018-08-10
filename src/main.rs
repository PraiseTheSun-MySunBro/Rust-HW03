mod solver;

use solver::maze_runner::MazeRunner;

fn main() {
    let maze_runner = MazeRunner::initialize(String::from("src/maps/ns100b.maze"));

    println!("{:?}", maze_runner.map);
}
