use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increasing() {
        let vec1 = vec![199_i64, 200,208,210,200,207,240,269,260,263];
        assert_eq!(nr_of_increased(&vec1), 7_i64);
    }
    #[test]
    fn count_increasing_averaged() {
        let vec1 = vec![199_i64, 200,208,210,200,207,240,269,260,263];
        assert_eq!(nr_of_increased_averaged(&vec1), 5_i64);
    }
}


pub fn nr_of_increased(values: &Vec<i64>) -> i64 {
    let  i2 = values.windows(2).filter(|y| y[0]<y[1]);
    i2.count().try_into().unwrap()
}

pub fn nr_of_increased_averaged(values: &Vec<i64>) -> i64 {
    let i2 = values.windows(3).map(|x| x.iter().sum::<i64>()).collect::<Vec<i64>>();
    let i3 = i2.windows(2).filter(|y| y[0]<y[1]).count();
    i3.try_into().unwrap()
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 1");
    let file = File::open("inputs/day1.txt").expect("Could not open");
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|x| match x {Ok(d) => d.parse::<i64>().unwrap(), Err(e)=>panic!("{:?}", e)}).collect::<Vec<i64>>();

    let part1 = nr_of_increased(&lines);
    println!("Part 1:\t {}", part1);

    let part2 = nr_of_increased_averaged(&lines);
    println!("Part 2:\t {}", part2);

    Ok(())
}
