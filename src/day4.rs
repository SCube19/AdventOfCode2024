use std::fs;
use crate::solver::Solver;

pub struct Day4;
impl Solver<i32, i32> for Day4 {
    fn solve_part1(&self) -> i32 {
        let binding = fs::read_to_string("input/day4.txt").unwrap();
        let grid: Vec<Vec<&str>> = binding.lines().map(|c| c.split("").collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
        
        let mut result: i32 = 0;
        for i in 0..grid.len() {
            for j in 0..grid.get(0).unwrap().len() {
                let character: &str = grid.get(i).unwrap().get(j).unwrap();
                if  character == "X" {
                    result += Day4::check_xmas(&grid, i, j);
                }
            }
        }
        return result;
    }

    fn solve_part2(&self) -> i32 {
        let binding = fs::read_to_string("input/day4.txt").unwrap();
        let grid: Vec<Vec<&str>> = binding.lines().map(|c| c.split("").collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
        
        let mut result: i32 = 0;
        for i in 0..grid.len() {
            for j in 0..grid.get(0).unwrap().len() {
                let character: &str = grid.get(i).unwrap().get(j).unwrap();
                if  character == "A" {
                    result += Day4::check_xmasx(&grid, i, j);
                }
            }
        }
        return result;
    }
}


impl Day4 {
    fn check_xmas(grid: &Vec<Vec<&str>>, i: usize, j: usize) -> i32 {
        let mut result: i32 = 0;

        if  *grid.get(i.overflowing_sub(1).0).unwrap_or(&vec![]).get(j).unwrap_or(&"x") == "M" &&
            *grid.get(i.overflowing_sub(2).0).unwrap_or(&vec![]).get(j).unwrap_or(&"x") == "A" &&
            *grid.get(i.overflowing_sub(3).0).unwrap_or(&vec![]).get(j).unwrap_or(&"x") == "S" {
                    result += 1;
            }

        if  *grid.get(i + 1).unwrap_or(&vec![]).get(j).unwrap_or(&"x") == "M" &&
            *grid.get(i + 2).unwrap_or(&vec![]).get(j).unwrap_or(&"x") == "A" &&
            *grid.get(i + 3).unwrap_or(&vec![]).get(j).unwrap_or(&"x") == "S" {
                    result += 1;
            }

        if  *grid.get(i).unwrap_or(&vec![]).get(j + 1).unwrap_or(&"x") == "M" &&
            *grid.get(i).unwrap_or(&vec![]).get(j + 2).unwrap_or(&"x") == "A" &&
            *grid.get(i).unwrap_or(&vec![]).get(j + 3).unwrap_or(&"x") == "S" {
               result += 1;
            }

        if  *grid.get(i).unwrap_or(&vec![]).get(j.overflowing_sub(1).0).unwrap_or(&"x") == "M" &&
            *grid.get(i).unwrap_or(&vec![]).get(j.overflowing_sub(2).0).unwrap_or(&"x") == "A" &&
            *grid.get(i).unwrap_or(&vec![]).get(j.overflowing_sub(3).0).unwrap_or(&"x") == "S" {
               result += 1;
            }

        if  *grid.get(i.overflowing_sub(1).0).unwrap_or(&vec![]).get(j.overflowing_sub(1).0).unwrap_or(&"x") == "M" &&
            *grid.get(i.overflowing_sub(2).0).unwrap_or(&vec![]).get(j.overflowing_sub(2).0).unwrap_or(&"x") == "A" &&
            *grid.get(i.overflowing_sub(3).0).unwrap_or(&vec![]).get(j.overflowing_sub(3).0).unwrap_or(&"x") == "S" {
               result += 1;
            }

        if  *grid.get(i + 1).unwrap_or(&vec![]).get(j.overflowing_sub(1).0).unwrap_or(&"x") == "M" &&
            *grid.get(i + 2).unwrap_or(&vec![]).get(j.overflowing_sub(2).0).unwrap_or(&"x") == "A" &&
            *grid.get(i + 3).unwrap_or(&vec![]).get(j.overflowing_sub(3).0).unwrap_or(&"x") == "S" {
               result += 1;
            }

        if  *grid.get(i.overflowing_sub(1).0).unwrap_or(&vec![]).get(j + 1).unwrap_or(&"x") == "M" &&
            *grid.get(i.overflowing_sub(2).0).unwrap_or(&vec![]).get(j + 2).unwrap_or(&"x") == "A" &&
            *grid.get(i.overflowing_sub(3).0).unwrap_or(&vec![]).get(j + 3).unwrap_or(&"x") == "S" {
               result += 1;
            }

        if  *grid.get(i + 1).unwrap_or(&vec![]).get(j + 1).unwrap_or(&"x") == "M" &&
            *grid.get(i + 2).unwrap_or(&vec![]).get(j + 2).unwrap_or(&"x") == "A" &&
            *grid.get(i + 3).unwrap_or(&vec![]).get(j + 3).unwrap_or(&"x") == "S" {
               result += 1;
            }

        return result;
    }

    fn check_xmasx(grid: &Vec<Vec<&str>>, i: usize, j: usize) -> i32 {
        let ms: i8 = (*grid.get(i + 1).unwrap_or(&vec![]).get(j + 1).unwrap_or(&"x") == "M") as i8 +
                     (*grid.get(i + 1).unwrap_or(&vec![]).get(j.overflowing_sub(1).0).unwrap_or(&"x") == "M") as i8 +
                     (*grid.get(i.overflowing_sub(1).0).unwrap_or(&vec![]).get(j + 1).unwrap_or(&"x") == "M") as i8 +
                     (*grid.get(i.overflowing_sub(1).0).unwrap_or(&vec![]).get(j.overflowing_sub(1).0).unwrap_or(&"x") == "M") as i8;
        
        let ss: i8 = (*grid.get(i + 1).unwrap_or(&vec![]).get(j + 1).unwrap_or(&"x") == "S") as i8 +
                     (*grid.get(i + 1).unwrap_or(&vec![]).get(j.overflowing_sub(1).0).unwrap_or(&"x") == "S") as i8 +
                     (*grid.get(i.overflowing_sub(1).0).unwrap_or(&vec![]).get(j + 1).unwrap_or(&"x") == "S") as i8 +
                     (*grid.get(i.overflowing_sub(1).0).unwrap_or(&vec![]).get(j.overflowing_sub(1).0).unwrap_or(&"x") == "S") as i8;
        

        if ss == 2 && ms == 2 && 
            *grid.get(i.overflowing_sub(1).0).unwrap_or(&vec![]).get(j.overflowing_sub(1).0).unwrap_or(&"x") !=
            *grid.get(i + 1).unwrap_or(&vec![]).get(j + 1).unwrap_or(&"y") {
                return 1;
            }
        return 0;
    }
}