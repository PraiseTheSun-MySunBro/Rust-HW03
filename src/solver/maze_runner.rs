use std::fs::File;
use std::io::prelude::*;
use CellType;
use Heading;

pub struct MazeRunner {
    pub map: Vec<Vec<i32>>,
    pub x: i32,
    pub y: i32
}

impl MazeRunner {
    pub fn initialize(filename: String) -> MazeRunner {
        let mut maze_runner = MazeRunner {
            map: Vec::new(),
            x: 0,
            y: 0,
        };

        let content = MazeRunner::get_file_content(filename);
        let mut line_number = 0;

        for file_line in content {
            if file_line.is_empty() {
                continue;
            }

            let mut line: Vec<i32> = Vec::new();

            for i in 0..file_line.len() {
                let cell_value = file_line.chars().nth(i).unwrap();

                if cell_value == 'X' {
                    line.push(CellType::Wall as i32);
                } else if cell_value == 'B' {
                    line.push(CellType::BeginPos as i32);
                    maze_runner.x = i as i32;
                    maze_runner.y = line_number;
                } else if cell_value == 'T' {
                    line.push(CellType::Treasure as i32);
                } else if cell_value >= '1' && cell_value <= '9' {
                    line.push(cell_value as i32 - '0' as i32);
                } else {
                    line.push(CellType::Wall as i32)
                }
            }

            line_number += 1;
            maze_runner.map.push(line);
        }

        return maze_runner;
    }

    fn get_file_content(filename: String) -> Vec<String> {
        let mut f = File::open(filename).expect("File not found!");

        let mut content: String = String::new();
        f.read_to_string(&mut content).expect("Something went wrong with file reading");

        return content.split('\n').map(|s| s.to_string()).collect();
    }

    pub fn move_to(&mut self, heading: Heading) -> bool {
        let x_usize = self.x as usize;
        let y_usize = self.y as usize;

        if heading == Heading::N && self.y > 0 && (*self.map.get(y_usize - 1).unwrap().get(x_usize).unwrap() >= 0 || *self.map.get(y_usize - 1).unwrap().get(x_usize).unwrap() == CellType::Treasure as i32) {
            self.y -= 1;
            return true;
        }
        if heading == Heading::S && y_usize < self.map.len() - 1 && (*self.map.get(y_usize + 1).unwrap().get(x_usize).unwrap() >= 0 || *self.map.get(y_usize + 1).unwrap().get(x_usize).unwrap() == CellType::Treasure as i32) {
            self.y += 1;
            return true;
        }
        if heading == Heading::W && self.x > 0 && (*self.map.get(y_usize).unwrap().get(x_usize - 1).unwrap() >= 0 || *self.map.get(y_usize).unwrap().get(x_usize - 1).unwrap() == CellType::Treasure as i32) {
            self.x -= 1;
            return true;
        }
        if heading == Heading::E && x_usize < self.map.get(y_usize).unwrap().len() - 1 && (*self.map.get(y_usize).unwrap().get(x_usize + 1).unwrap() >= 0 || *self.map.get(y_usize).unwrap().get(x_usize + 1).unwrap() == CellType::Treasure as i32) {
            self.x += 1;
            return true;
        }

        return false;
    }

    pub fn scan(&self) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut i = self.y - 1;
        while i < self.y + 2 {
            let mut line: Vec<i32> = Vec::new();

            let mut j = self.x - 1;
            while j < self.x + 2 {
                let i_usize = i as usize;
                let j_usize = j as usize;

                if i >= 0 && self.map.len() as i32 > i && j >= 0 && self.map.get(i_usize).unwrap().len() as i32 > j {
                    line.push(*self.map.get(i_usize).unwrap().get(j_usize).unwrap());
                } else {
                    line.push(CellType::Wall as i32);
                }
                j += 1
            }
            result.push(line);
            i += 1;
        }

        return result;
    }

    pub fn get_position(&self) -> (i32, i32) {
        return (self.x, self.y);
    }

    pub fn get_size(&self) -> (i32, i32) {
        if self.map.len() == 0 || self.map.get(0).is_none() {
            panic!("Map has not provided");
        }
        return (self.map.len() as i32, self.map.get(0).unwrap().len() as i32);
    }
}