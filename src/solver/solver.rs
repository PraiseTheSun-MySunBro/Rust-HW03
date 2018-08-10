use CellType;
use Heading;
use solver::maze_runner::MazeRunner;

pub struct Solver<'a> {
    pub maze_runner: &'a mut MazeRunner,
}

impl<'a> Solver<'a> {
    pub fn initialize(runner: &mut MazeRunner) -> Solver {
        let solver = Solver {
            maze_runner: runner
        };

        return solver;
    }

    pub fn find_best_path(&mut self) -> Option<Vec<Heading>> {
        if self.maze_runner.get_size().0 == 0 || self.maze_runner.get_size().1 == 0 {
            return None;
        }

        let mut best_path: Vec<Heading> = Vec::new();
        return Some(best_path);
    }
}