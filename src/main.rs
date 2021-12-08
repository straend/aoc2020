#[allow(dead_code, unused_imports)]

use std::io;

mod helpers;
// import each day with mod dayX;
mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;


fn main() -> io::Result<()> {
    let day = std::env::args().nth(1).expect("No day given");
    match day.parse::<i32>().unwrap() {
        0 => day0::run()?,
        // Add a match for each day
        // N   =>  dayN::run()?,
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        4 => day4::run()?,
        5 => day5::run()?,
        6 => day6::run()?,
        7 => day7::run()?,
        8 => day8::run()?,

        _ => println!("Not implemented")
    }

    Ok(())
}
