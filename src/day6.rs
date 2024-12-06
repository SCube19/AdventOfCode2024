use std::fs;
use crate::solver::Solver;

pub struct Day6;
impl Solver<i32, i32> for Day6 {
    fn solve_part1(&self) -> i32 {
        let file: String = fs::read_to_string("input/day6.txt").unwrap();
        let grid: Vec<Vec<&str>> = file.lines().map(|c| c.split("").collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
        
        let mut guardCoord: (usize, usize) = (0, 0);
        for i in 0..grid.len(){
            for j in 0..grid.get(0).unwrap().len() {
                if *grid.get(i).unwrap().get(j).unwrap() == "^" {
                    guardCoord = (i, j);
                }
            }
        }

        let result: i32 = Day6::simulate(&grid, &mut guardCoord);
        return result;
    }

    fn solve_part2(&self) -> i32 {
        let file: String = fs::read_to_string("input/day6.txt").unwrap();

        return 0;
    }
}

enum Direction {
    UP, 
    DOWN,
    LEFT,
    RIGHT
}

impl Day6 {


    fn simulate(grid: &Vec<Vec<&str>>, guardCoord: &mut (usize, usize)) -> i32 {
        let mut result: i32 = 0;
        let mut direction: Direction = Direction::UP;

        while grid.get(guardCoord.0).unwrap_or(&vec![]).get(guardCoord.1).unwrap_or(&"") != "" {
            match direction {
                Direction::UP => todo!(),
                Direction::DOWN => todo!(),
                Direction::LEFT => todo!(),
                Direction::RIGHT => todo!(),
            }
        }
        return result;
    }
}