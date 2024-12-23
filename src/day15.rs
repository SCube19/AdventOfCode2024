use std::{env::current_exe, fs};
use crate::solver::Solver;

enum Move {
    Up,
    Down,
    Left,
    Right
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Cell {
    Empty,
    Robot,
    Wall,
    Box
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum CellWide {
    Empty,
    Robot,
    Wall,
    BoxLeft,
    BoxRight
}

pub struct Day15;
impl Solver<i32, i32> for Day15 {
    fn solve_part1(&self) -> i32 {
        let file: String = fs::read_to_string("input/day15.txt").unwrap();
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        let mut movements: Vec<Move> = Vec::new();

        let mut is_movements: bool = false;
        
        for line in file.lines() {
            if line.is_empty() {
                is_movements = true;
                continue;
            }
            
            if !is_movements {
                grid.push(line.chars().map(|x| match x {
                    '@' => Cell::Robot,
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'O' => Cell::Box,
                    _ => panic!("")
                }).collect())
            }
            else {
                for c in line.chars() {
                    movements.push(match c {
                        '>' => Move::Right,
                        '<' => Move::Left,
                        '^' => Move::Up,
                        'v' => Move::Down,
                        _ => panic!("")
                    })
                }
            }
        }
        
        let mut robot_position: (usize, usize) = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == Cell::Robot {
                    robot_position = (i, j);
                    grid[i][j] = Cell::Empty;
                }
            }
        }
        
        for movement in movements {
           // Day15::print_grid(&grid);
            let next_position: (usize, usize) = Day15::get_next_position(robot_position, &movement);
            if grid.get(next_position.0).unwrap_or(&vec![]).get(next_position.1).is_none() {
                continue;
            }
        
            Day15::recursive_move(&movement, &mut grid, next_position.clone());
            
            if grid[next_position.0][next_position.1] != Cell::Empty {
                continue;
            }

            grid[robot_position.0][robot_position.1] = Cell::Empty;
            robot_position = next_position;
            grid[robot_position.0][robot_position.1] = Cell::Robot;
        }

        let mut result: i32 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == Cell::Box {
                    result += 100 * i as i32 + j as i32;
                }
            }
        }
        return result;
    }

    fn solve_part2(&self) -> i32 {
        let file: String = fs::read_to_string("input/day15.txt").unwrap();
        let mut grid: Vec<Vec<CellWide>> = Vec::new();
        let mut movements: Vec<Move> = Vec::new();

        let mut is_movements: bool = false;
        
        for line in file.lines() {
            if line.is_empty() {
                is_movements = true;
                continue;
            }
            
            if !is_movements {
                grid.push(line.chars().map(|x| match x {
                    '@' => (CellWide::Robot, CellWide::Empty),
                    '#' => (CellWide::Wall, CellWide::Wall),
                    '.' => (CellWide::Empty, CellWide::Empty),
                    'O' => (CellWide::BoxLeft, CellWide::BoxRight),
                    _ => panic!("")
                }).flatten().collect())
            }
            else {
                for c in line.chars() {
                    movements.push(match c {
                        '>' => Move::Right,
                        '<' => Move::Left,
                        '^' => Move::Up,
                        'v' => Move::Down,
                        _ => panic!("")
                    })
                }
            }
        }
        
        let mut robot_position: (usize, usize) = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == Cell::Robot {
                    robot_position = (i, j);
                    grid[i][j] = Cell::Empty;
                }
            }
        }
        
        for movement in movements {
           // Day15::print_grid(&grid);
            let next_position: (usize, usize) = Day15::get_next_position(robot_position, &movement);
            if grid.get(next_position.0).unwrap_or(&vec![]).get(next_position.1).is_none() {
                continue;
            }
        
            Day15::recursive_move(&movement, &mut grid, next_position.clone());
            
            if grid[next_position.0][next_position.1] != Cell::Empty {
                continue;
            }

            grid[robot_position.0][robot_position.1] = Cell::Empty;
            robot_position = next_position;
            grid[robot_position.0][robot_position.1] = Cell::Robot;
        }

        let mut result: i32 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == Cell::Box {
                    result += 100 * i as i32 + j as i32;
                }
            }
        }
        return result;
    }
}


impl Day15 {
    fn get_next_position(current_position: (usize, usize), movement: &Move) -> (usize, usize) {
        return match movement {
            Move::Up => (current_position.0.overflowing_sub(1).0, current_position.1),
            Move::Down => (current_position.0 + 1, current_position.1),
            Move::Left => (current_position.0, current_position.1.overflowing_sub(1).0),
            Move::Right => (current_position.0, current_position.1 + 1),
        }
    }

    fn recursive_move(movement: &Move, grid: &mut Vec<Vec<Cell>>, current_position: (usize, usize)) {
        if grid.get(current_position.0).unwrap_or(&vec![]).get(current_position.1).is_none() {
            return;
        }

        if grid[current_position.0][current_position.1] == Cell::Wall  || grid[current_position.0][current_position.1] == Cell::Empty {
            return;
        }

        if grid[current_position.0][current_position.1] == Cell::Box {
            let next_position: (usize, usize) = Day15::get_next_position(current_position, &movement);
            Day15::recursive_move(movement, grid, next_position);
            if grid.get(next_position.0).unwrap_or(&vec![]).get(next_position.1).is_none() {
                return;
            }
            if grid[next_position.0][next_position.1] != Cell::Empty {
                return;
            }
            grid[next_position.0][next_position.1] = Cell::Box;
            grid[current_position.0][current_position.1] = Cell::Empty;
        }
    }

    fn print_grid(grid: &Vec<Vec<Cell>>) {
        for line in grid {
            for cell in line {
                print!("{}", match cell {
                    Cell::Empty => '.',
                    Cell::Wall => '#',
                    Cell::Robot => '@',
                    Cell::Box => 'O'
                })
            }
            println!("");
        }
        println!("");
    }
}
