use std::io;

use crate::helpers;

use std::str::FromStr;
use core::cmp::max;

#[derive(Debug, PartialEq)]
struct Point{
    x: i64,
    y: i64,
}
#[derive(Debug, PartialEq)]
struct Line{
    from: Point,
    to: Point,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str)-> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(',').collect();
        match v.len() {
            2 => Ok(Point{x: v[0].trim().parse::<i64>().unwrap(), y: v[1].trim().parse::<i64>().unwrap() }),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct ParseError;
impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str)-> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split("->").collect();
        let p1:Point = v[0].parse().unwrap();
        let p2:Point = v[1].parse().unwrap();
        Ok(Line{from: p1, to: p2})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_point() {
        let p1: Point = "0,9".parse().unwrap();
        assert_eq!(p1, Point{x: 0, y: 9});
        let p1: Point = "0,20".parse().unwrap();
        assert_eq!(p1, Point{x: 0, y: 20});
    }

    #[test]
    fn parse_line() {
        let p1: Line = "0,9 -> 0,20".parse().unwrap();
        assert_eq!(p1, Line{from: Point{x: 0, y: 9}, to: Point{x: 0, y:20} });
        let p1: Line = "0 , 9 ->  0,20".parse().unwrap();
        assert_eq!(p1, Line{from: Point{x: 0, y: 9}, to: Point{x: 0, y:20} });
    }

    #[test]
    fn get_straight_line_vents() {
        let inp = helpers::read_file_to_vec::<Line>("inputs/day5_test.txt");
        let only_straight:Vec<&Line> = inp.iter().filter(|x| x.from.x == x.to.x || x.from.y == x.to.y).collect();
        assert_eq!(get_vents(only_straight), 5);
    }

    #[test]
    fn get_all_line_vents() {
        let inp = helpers::read_file_to_vec::<Line>("inputs/day5_test.txt");
        let all_lines:Vec<&Line> =inp.iter().collect();
        assert_eq!(get_vents(all_lines), 12);
    }

}
fn get_vents(vents: Vec<&Line>) -> u64 {
    // Get size of map
    let max_x = max(vents.iter().max_by_key(|x| x.from.x).unwrap().from.x, vents.iter().max_by_key(|x| x.to.x).unwrap().to.x);
    let max_y = max(vents.iter().max_by_key(|x| x.from.y).unwrap().from.y, vents.iter().max_by_key(|x| x.to.y).unwrap().to.y);

    let mut lines = vec![0; ((max_x+1) * (max_y+1)) as usize];

    for l in vents.iter() {
        let x_step = match l.from.x > l.to.x {
            true => -1,
            _ => 1,
        };
        let y_step = match l.from.y > l.to.y {
            true => -1,
            _ => 1,
        };
        let x_iterator = num::range_step_inclusive(l.from.x, l.to.x, x_step);
        let y_iterator = num::range_step_inclusive(l.from.y, l.to.y, y_step);

        if l.from.x == l.to.x {
            // Straight line on X
            for y in y_iterator {
                lines[(y* (max_y+1) + l.from.x) as usize]+=1;
            }
        } else if l.from.y == l.to.y {
            // Straight line on Y
            for x in x_iterator {
                lines[(l.from.y* (max_y+1) + x) as usize]+=1;
            }
        } else {
            // Diagonal line, provided as 45° == as many x as y steps

            for (xx, yy) in x_iterator.zip(y_iterator){
                lines[(yy* (max_y+1) + xx) as usize]+=1;
            }
        }

    }
    let su:usize = lines.iter().filter(|&x| x>=&2).count();
    su as u64
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 5");

    let inp = helpers::read_file_to_vec::<Line>("inputs/day5.txt");
    let all_lines:Vec<&Line> =inp.iter().collect();
    let only_straight:Vec<&Line> = inp.iter().filter(|x| x.from.x == x.to.x || x.from.y == x.to.y).collect();

    println!("Part 1: {}", get_vents(only_straight));
    println!("Part 2: {}", get_vents(all_lines));

    Ok(())
}
