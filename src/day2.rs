use std::io;

use crate::helpers;

use std::str::FromStr;

#[derive(Debug,PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    steps: i64,
}

#[derive(Debug, PartialEq)]
struct Point{
    x: i64,
    y: i64,
}
#[derive(Debug, PartialEq)]
struct Point2{
    x: i64,
    y: i64,
    aim: i64,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str)-> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
struct ParseError;
impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str)-> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(' ').collect();

        let d:Direction = match v[0].parse() {
            Ok(x) => x,
            _ => return Err(ParseError),
        };

        let steps: i64 = match v[1].parse() {
            Ok(x) => x,
            _ => return Err(ParseError),
        };

        Ok(Instruction{direction: d, steps: steps})
    }
}

fn calc_position(input: &Vec<Instruction>) -> Point {
    input.iter().fold(Point{x:0, y: 0}, |mut acc, x| {
        match x.direction {
            Direction::Forward => acc.x += x.steps,
            Direction::Down => acc.y += x.steps,
            Direction::Up => acc.y -= x.steps,
        };
        acc
    })
}
fn calc_position2(input: &Vec<Instruction>) -> Point2 {
    input.iter().fold(Point2{x:0, y: 0, aim: 0}, |mut acc, x| {
        match x.direction {
            Direction::Forward => {
                acc.x += x.steps;
                acc.y += acc.aim * x.steps;
            },
            Direction::Down => acc.aim += x.steps,
            Direction::Up => acc.aim -= x.steps,
        };
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        let x: Instruction = "forward 5".parse().unwrap();
        assert_eq!(x, Instruction{direction: Direction::Forward, steps: 5});
        let x: Instruction = "down 50".parse().unwrap();
        assert_eq!(x, Instruction{direction: Direction::Down, steps: 50});
    }
    #[test]
    fn check_enginge(){
        let inp = helpers::read_file_to_vec::<Instruction>("inputs/day2_test.txt");
        let res = calc_position(&inp);
        assert_eq!(res, Point{x: 15, y: 10});
    }

    #[test]
    fn check_enginge2(){
        let inp = helpers::read_file_to_vec::<Instruction>("inputs/day2_test.txt");
        let res = calc_position2(&inp);
        assert_eq!(res, Point2{x: 15, y: 60, aim: 10});
    }

}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 2");

    let inp = helpers::read_file_to_vec::<Instruction>("inputs/day2.txt");
    let res = calc_position(&inp);

    println!("Part1: {:?}", res.x*res.y);

    let res = calc_position2(&inp);
    println!("Part2: {:?}", res.x*res.y);

    Ok(())
}
