use std::fs;
use crate::solver::Solver;
use regex::Regex;

pub struct Day3;
impl Solver<i32, i32> for Day3 {
    fn solve_part1(&self) -> i32 {
        let re = Regex::new(r"mul\([0-9]{1, 3},[0-9]{1, 3}\)").unwrap();
        let file = fs::read_to_string("input/day3.txt").unwrap();

        return re.find_iter(file.as_str()).map(|mul| {
            let slice: &str = &mul.as_str()[4..mul.len() - 1];
            return slice.split(',').fold(1, |acc: i32, x: &str| acc * x.parse::<i32>().unwrap());
        }).fold(0, |acc, x| acc + x);
    }

    fn solve_part2(&self) -> i32 {
        let re: Regex = Regex::new(r"(mul\([0-9]{1, 3},[0-9]{1, 3}\)|do\(\)|don't\(\))").unwrap();
        let file: String = fs::read_to_string("input/day3.txt").unwrap();

        let mut result: i32 = 0;
        let mut enabled: bool = true;
        for m in re.find_iter(file.as_str()) {
            match m.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                mul => if enabled {
                    let slice: &str = &mul[4..mul.len() - 1];
                    result += slice.split(",").fold(1, |acc: i32, x: &str| acc * x.parse::<i32>().unwrap());
                }
            }
        }
        return result;
    }
}
