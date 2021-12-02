#[allow(dead_code, unused_imports)]

use std::io;

mod helpers;
// import each day with mod dayX;
mod day0;
mod day1;
mod day2;


fn main() -> io::Result<()> {
    let day = std::env::args().nth(1).expect("No day given");
    match day.parse::<i32>().unwrap() {
        0 => day0::run()?,
        // Add a match for each day
        // N   =>  dayN::run()?,
        1 => day1::run()?,
        2 => day2::run()?,

        _ => println!("Not implemented")
    }

    Ok(())
}
