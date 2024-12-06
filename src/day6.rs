use std::{collections::HashSet, fs};
use crate::solver::Solver;
pub struct Day6;
impl Solver<i32, i32> for Day6 {
    fn solve_part1(&self) -> i32 {
        let file: String = fs::read_to_string("input/day6.txt").unwrap();
        let grid: Vec<Vec<&str>> = file.lines().map(|c| c.split("").collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
        
        let mut guard_coord: (usize, usize) = (0, 0);
        for i in 0..grid.len(){
            for j in 0..grid.get(0).unwrap().len() {
                if *grid.get(i).unwrap().get(j).unwrap() == "^" {
                    guard_coord = (i, j);
                }
            }
        }

        let result: i32 = Day6::simulate(&grid, &guard_coord).unwrap().into_iter().flatten().fold(0, |acc: i32, x: bool| acc + (x as i32));
        return result;
    }

    fn solve_part2(&self) -> i32 {
        let file: String = fs::read_to_string("input/day6.txt").unwrap();
        let mut grid: Vec<Vec<&str>> = file.lines().map(|c| c.split("").collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
        
        let mut guard_coord: (usize, usize) = (0, 0);
        for i in 0..grid.len(){
            for j in 0..grid.get(0).unwrap().len() {
                if *grid.get(i).unwrap().get(j).unwrap() == "^" {
                    guard_coord = (i, j);
                }
            }
        }

        let mut result: i32 = 0;
        let path: Vec<Vec<bool>> = Day6::simulate(&grid, &guard_coord).unwrap();
        for i  in 0..path.len() {
            for j in 0..path[0].len() {
                if guard_coord == (i, j) {
                    continue;
                }
                if path[i][j] {
                    grid[i][j] = "#";
                    if Day6::simulate(&grid, &guard_coord).is_err() {
                        result += 1;
                    }
                    grid[i][j] = ".";
                }
            }
        }
        return result;
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    UP, 
    DOWN,
    LEFT,
    RIGHT
}

impl Day6 {
    fn simulate(grid: &Vec<Vec<&str>>, coord: &(usize, usize)) -> Result<Vec<Vec<bool>>, ()> {
        let mut direction: Direction = Direction::UP;
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        let mut guard_coord = *coord;

        let mut steps: i32 = 0;

        while grid.get(guard_coord.0).unwrap_or(&vec![]).get(guard_coord.1).unwrap_or(&"") != &"" {
            visited[guard_coord.0][guard_coord.1] = true;
              
            let next: (usize, usize) = match direction {
                Direction::UP => (guard_coord.0.wrapping_sub(1), guard_coord.1),
                Direction::DOWN => (guard_coord.0 + 1, guard_coord.1),
                Direction::LEFT => (guard_coord.0, guard_coord.1.wrapping_sub(1)),
                Direction::RIGHT => (guard_coord.0, guard_coord.1 + 1),
            };

            let next_cell: &str = grid.get(next.0).unwrap_or(&vec![]).get(next.1).unwrap_or(&"");
            if  next_cell == "#" {
                match direction {
                    Direction::UP => direction = Direction::RIGHT,
                    Direction::DOWN => direction = Direction::LEFT,
                    Direction::LEFT => direction = Direction::UP,
                    Direction::RIGHT => direction = Direction::DOWN,
                }
            }
            else {
                guard_coord = next;
            }
            steps += 1;
            if steps > 130 * 130 * 4 {
                return Err(());
            }
        }
        return Ok(visited);
    }
}
