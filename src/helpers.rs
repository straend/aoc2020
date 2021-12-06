use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use std::fmt;

pub fn read_file_to_vec<A> (filename: &str) -> Vec<A>
where
    A: FromStr,
    <A as FromStr>::Err: fmt::Debug,
{
    let file = File::open(filename).expect("Could not open");
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|x| match x {Ok(d) => d.parse::<A>().unwrap(), Err(e)=>panic!("{:?}", e)}).collect::<Vec<A>>();

    lines
}

pub fn read_line_to_vec<A> (filename: &str) -> Vec<A>
where
    A: FromStr,
    <A as FromStr>::Err: fmt::Debug,
{
    let mut file = File::open(filename).expect("Could not open");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Could not read to string");

    s.split(',').map( |x| x.parse::<A>().unwrap() ).collect::<Vec<A>>()

}