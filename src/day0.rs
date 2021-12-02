use std::io;
//use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_even_number() {
        assert_eq!(false, even_number(1));
        assert_eq!(true, even_number(2));
        assert_eq!(false, even_number(3));
        assert_eq!(true, even_number(4));
        assert_eq!(false, even_number(5));
    }
}


pub fn even_number(number: i32) -> bool {
    number %2 == 0
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 0\n");

    println!("2 is even: {}", even_number(2));

    Ok(())
}
