use std::fs::File;
use std::io::prelude::*;

pub enum CellType {
    Treasure = -2, Wall = -1, BeginPos = 0
}

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
}