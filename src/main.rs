use solver::Solver;

mod solver;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
use crate::day1::Day1;
use crate::day2::Day2;
use crate::day3::Day3;
use crate::day4::Day4;
use crate::day5::Day5;

fn main() {
    let solution1: Day1 = Day1;
    solution1.solve();

    let solution2: Day2 = Day2;
    solution2.solve();

    let solution3: Day3 = Day3;
    solution3.solve();

    let solution4: Day4 = Day4;
    solution4.solve();

    let solution5: Day5 = Day5;
    solution5.solve();
}
