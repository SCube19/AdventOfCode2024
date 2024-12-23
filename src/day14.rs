
use std::{fs, io::Write};
use crate::solver::Solver;
use regex::Regex;

pub struct Day14;
impl Solver<i32, String> for Day14 {
    fn solve_part1(&self) -> i32 {
        let file: String = fs::read_to_string("input/day14.txt").unwrap();
        let values: Vec<i32> = Regex::new(r"-?\d+").unwrap().find_iter(file.as_str()).into_iter()
                                                .map(|x| x.as_str().parse::<i32>().unwrap()).collect();
        
        const H: usize = 103;
        const W: usize = 101;
        const SECONDS: i32 = 100;

        let mut grid: Vec<Vec<i32>> = vec![vec![0; W]; H];

        let mut i: usize = 0;
        while i < values.len() {
            let px: i32 = values[i];
            let py: i32 = values[i + 1];
            let vx: i32 = values[i + 2];
            let vy: i32 = values[i + 3];
            i += 4;

            let last_position_x: usize = (px + vx * SECONDS).rem_euclid(W as i32) as usize;
            let last_position_y: usize = (py + vy * SECONDS).rem_euclid(H as i32) as usize;

            grid[last_position_y][last_position_x] += 1;
        }

        let result = grid[0..(H / 2)].iter().fold(0, |acc, x| acc + x[0..(W / 2)].iter().fold(0, |acc, y| acc + y)) * 
                     grid[0..(H / 2)].iter().fold(0, |acc, x| acc + x[(W / 2 + 1)..].iter().fold(0, |acc, y| acc + y)) * 
                     grid[(H / 2 + 1)..].iter().fold(0, |acc, x| acc + x[0..(W / 2)].iter().fold(0, |acc, y| acc + y)) * 
                     grid[(H / 2 + 1)..].iter().fold(0, |acc, x| acc + x[(W / 2 + 1)..].iter().fold(0, |acc, y| acc + y));

        return result;
    }

    fn solve_part2(&self) -> String {
       let file: String = fs::read_to_string("input/day14.txt").unwrap();
        let values: Vec<i32> = Regex::new(r"-?\d+").unwrap().find_iter(file.as_str()).into_iter()
                                                .map(|x| x.as_str().parse::<i32>().unwrap()).collect();
        
        const H: usize = 103;
        const W: usize = 101;
        const SECONDS: i32 = 10000;

        let mut guards: Vec<(i32, i32, i32, i32)> = Vec::new();
        
        let mut i: usize = 0;
        while i < values.len() {
            let px: i32 = values[i];
            let py: i32 = values[i + 1];
            let vx: i32 = values[i + 2];
            let vy: i32 = values[i + 3];
            i += 4;
            
            guards.push((px, py, vx, vy));
        }
        
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("output/day14.txt")
            .unwrap();

        for second in 0..SECONDS {
            let mut grid: Vec<Vec<i32>> = vec![vec![0; W]; H];
            
            for guard in guards.iter_mut() {
                guard.0 = (guard.0 + guard.2).rem_euclid(W as i32);
                guard.1 = (guard.1 + guard.3).rem_euclid(H as i32);

                grid[guard.1 as usize][guard.0 as usize] += 1;
            }
            
            let mut shape: String = String::new();
            shape += &format!("Second: {}\n", second);

            for line in grid {
                for val in line {
                    if val > 0 {
                        shape += "🟩";
                    }
                    else {
                        shape += " ";
                    }
                }
                shape += "\n";
            }
            file.write(shape.as_bytes()).unwrap();
        }

        return String::from("Visual puzzle uncomment the code to see the result");
    }
}
