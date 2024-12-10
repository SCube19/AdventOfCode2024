
use std::fs;
use crate::solver::Solver;

pub struct DoubleList<T> {
    next: Option<Box<DoubleList<T>>>,
    prev: Option<Box<DoubleList<T>>>,
    value: T,
}


pub struct Day9;
impl Solver<i64, String> for Day9 {
    fn solve_part1(&self) -> i64 {
        let file: String = fs::read_to_string("input/day9.txt").unwrap();
        let bytes: Vec<char> = file.chars().collect();

        let mut diskspace: Vec<String> = Day9::create_diskspace(&bytes);
        Day9::consolidate(&mut diskspace);

        return Day9::get_checksum(&diskspace);
    }

    fn solve_part2(&self) -> String {
        // let file: String = fs::read_to_string("input/day9.txt").unwrap();
        // let bytes: Vec<char> = file.chars().collect();

        // let block_lists: (DoubleList<(i32, i32)>, DoubleList<(i32, i32, String)>) = Day9::create_blocks(&bytes);
        // Day9::consolidate_blocks(&block_lists.0, &block_lists.1);
        // let diskspace: Vec<String> = Day9::create_diskspace(&block_lists.0, &block_lists.1);

        // return Day9::get_checksum(&diskspace)
        return String::from("Ain't doing that in rust");
    }
}

impl Day9 {
    fn get_checksum(diskspace: &Vec<String>) -> i64 {
        let mut checksum: i64 = 0;
        for i in 0..diskspace.len() {
            if diskspace[i] == "." {
                continue;
            }
            checksum += i as i64 * diskspace[i].parse::<i64>().unwrap();
        }
        return checksum;   
    }

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

    // fn create_blocks(bytes: &Vec<char>) -> (DoubleList<(i32, i32)>, DoubleList<(i32, i32, i32)>) {
    //     let mut free_blocks: Option<Box<DoubleList<(i32, i32)>>> = None;
    //     let mut data_blocks: Option<Box<DoubleList<(i32, i32, String)>>> = None;

    //     let mut id: u32 = 0;
    //     let mut free_flag: bool = false;
    //     for byte in bytes {
    //         let amount: usize = byte.to_digit(10).unwrap() as usize;
    //         if free_flag {
    //             free_blocks
    //             diskspace.extend(std::iter::repeat_n(".".to_string(), amount));
    //         }
    //         else {
    //             let strid = id.to_string();
    //             diskspace.extend(std::iter::repeat_n(strid, amount));
    //             id += 1;
    //         }            
    //         free_flag = !free_flag;
    //     }

    //     return (free_blocks, data_blocks);
    // }
}
