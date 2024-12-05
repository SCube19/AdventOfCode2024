#![feature(core_intrinsics)]

use std::intrinsics::type_name;
pub trait Solver<T: std::fmt::Display , U: std::fmt::Display> {
    fn solve_part1(&self) -> T;
    fn solve_part2(&self) -> U;
    fn solve(&self) -> () {
        unsafe {
            println!("{} part1: {}, part2: {}", type_name::<Self>() ,self.solve_part1(), self.solve_part2());
        }
    }
}