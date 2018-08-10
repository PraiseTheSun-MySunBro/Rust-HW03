mod solver;

use solver::maze_runner::MazeRunner;
use solver::solver::Solver;

#[derive(PartialEq)]
pub enum CellType {
    Treasure = -2, Wall = -1, BeginPos = 0
}

#[derive(PartialEq)]
pub enum Heading {
    N, S, W, E
}

fn main() {
    let mut maze_runner = MazeRunner::initialize(String::from("src/maps/ns100b.maze"));
    {
        let mut solver = Solver::initialize(&mut maze_runner);
        println!("{:?}", solver.maze_runner.scan());
        solver.maze_runner.move_to(Heading::S);
    }

    println!("{:?}", maze_runner.scan());
}
