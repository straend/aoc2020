use std::io;

use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_fish() {
        let data:Vec<usize> = vec![3,4,3,1,2];
        let s = lanternfish(&data, 80);
        assert_eq!(s, 5934);
    }
}

fn lanternfish(inp: &Vec<usize>, days: usize) -> u64 {
    let mut counts: Vec<usize> = vec![0; 9];

    for i in 0..6 {
        counts[i] = inp.iter().filter(|&x| x==&i).count();
    };

    for _day in 1..=days {
        //check newbirths
        let k = counts[0];
        // Rotate all one step left
        // but remember that 0 resets to 6 (and spawns a fish at 8)
        // and 7 goes to 6 -> so 6 = last0 + last7
        counts[0] = counts[1];
        counts[1] = counts[2];
        counts[2] = counts[3];
        counts[3] = counts[4];
        counts[4] = counts[5];
        counts[5] = counts[6];
        counts[6] = k + counts[7];
        counts[7] = counts[8];
        counts[8] = k;
    }

    counts.iter().sum::<usize>() as u64
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 6");
    let lines = helpers::read_line_to_vec::<usize>("inputs/day6.txt");

    println!("Part 1:\t {}", lanternfish(&lines, 80));
    println!("Part 2:\t {}", lanternfish(&lines, 256));

    Ok(())
}
