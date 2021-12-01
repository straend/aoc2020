use std::fs::File;
use std::io::{BufRead, BufReader};
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