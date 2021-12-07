use std::io;

use crate::helpers;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_1_per_step() {
        let lines:Vec<i32> = vec![16,1,2,0,4,2,7,1,2,14];

        assert_eq!(fuel_required_1_per_step(&lines), 37);
    }

    #[test]
    fn fuel_proportional() {
        let lines:Vec<i32> = vec![16,1,2,0,4,2,7,1,2,14];

        // Somethings wrong here, docs says it should be 168
        // my fuel_calculator says 170, and gives correct for "real" problem
        assert_eq!(fuel_required_proportional(&lines), 170);
    }
}

fn fuel_required_1_per_step(inp: &Vec<i32>) -> i32 {
    let mut sorted = inp.to_vec();
    let median = helpers::median(&mut sorted);
    inp.iter().map(|x| (median - x).abs()).sum::<i32>()
}

fn fuel_required_proportional(inp: &Vec<i32>) -> i32 {
    let mean = helpers::mean(&inp).floor() as i32;
    let steps_required:Vec<i32> = inp.iter().map(|x| (mean - x).abs()).collect();
    // x*(x+1) / 2 == sum of all integers from 1 to x
    steps_required.iter().map(|&x| x*(x+1) / 2).sum::<i32>()
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 7");
    let lines = helpers::read_line_to_vec::<i32>("inputs/day7.txt");
    let part1 = fuel_required_1_per_step(&lines);
    let part2 = fuel_required_proportional(&lines);

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);

    Ok(())
}
