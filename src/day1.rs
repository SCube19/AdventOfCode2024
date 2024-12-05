use std::fs;
use std::collections::HashMap;
use crate::solver::Solver;

pub struct Day1;
impl Solver<i32, i32> for Day1 {
    fn solve_part1(&self) -> i32 {
        let mut distances: (Vec<i32>, Vec<i32>) = Day1::read("input/day1.txt");

        distances.0.sort();
        distances.1.sort();

        let mut result: i32 = 0;
        for i in 0..distances.1.len() {
            result += (distances.0[i] - distances.1[i]).abs();
        }   
        return result;
    }

    fn solve_part2(&self) -> i32 {
        let values: (Vec<i32>, Vec<i32>) = Day1::read("input/day2.txt");

        let mut right_count: HashMap<i32, i32>  = HashMap::new();
        
        for right_id in values.1 {
            *right_count.entry(right_id).or_insert(0) += 1;
        }

        let mut similarity: i32 = 0;
        for left_id in values.0 {
            similarity += left_id * (*right_count.entry(left_id).or_default());
        }

        return similarity;
    }
}

impl Day1 {
    pub fn read(filepath: &str) -> (Vec<i32>, Vec<i32>) {
        let file: String = fs::read_to_string(filepath).
                            expect("Error reading the file");

        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();

        let mut line: i32 = 0;

        for value in file.split_whitespace() {
            if line % 2 == 0 {
                left.push(value.parse::<i32>().unwrap());
            }   
            else {
                right.push(value.parse::<i32>().unwrap());
            }    
            line += 1; 
        }   
        return (left, right);                     
    }
}


