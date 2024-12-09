
use std::fs;
use crate::solver::Solver;

pub struct Day9;
impl Solver<i64, i64> for Day9 {
    fn solve_part1(&self) -> i64 {
        let file: String = fs::read_to_string("input/day9.txt").unwrap();
        let bytes: Vec<char> = file.chars().collect();

        let mut diskspace: Vec<String> = Day9::create_diskspace(&bytes);
        Day9::consolidate(&mut diskspace);

        let mut checksum: i64 = 0;
        for i in 0..diskspace.len() {
            if (diskspace[i] == ".") {
                continue;
            }
            checksum += i as i64 * diskspace[i].parse::<i64>().unwrap();
        }
        return checksum;   
    }

    fn solve_part2(&self) -> i64 {
        let file: String = fs::read_to_string("input/day9.txt").unwrap();

        return 0;
    }
}

impl Day9 {
    fn create_diskspace(bytes: &Vec<char>) -> Vec<String> {
        let mut diskspace: Vec<String> = Vec::new();
        let mut id: u32 = 0;
        let mut free_flag: bool = false;
        for byte in bytes {
            let amount: usize = byte.to_digit(10).unwrap() as usize;
            if free_flag {
                diskspace.extend(std::iter::repeat_n(".".to_string(), amount));
            }
            else {
                let strid = id.to_string();
                diskspace.extend(std::iter::repeat_n(strid, amount));
                id += 1;
            }            
            free_flag = !free_flag;
        }
        return diskspace;
    }

    fn consolidate(diskspace: &mut Vec<String>) -> () {
        let mut left = 0;
        let mut right = diskspace.len() - 1;

        while left < right {
            while diskspace[left] != "." && left < right {
                left += 1;
            }
            while diskspace[right] == "." && left < right {
                right -= 1;
            }
            diskspace.swap(left, right);
        }
    }
}
