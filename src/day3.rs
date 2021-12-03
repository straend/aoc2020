use std::io;
use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_consumption(){
        let inp = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");
        let (gamma, epsilon) = get_gamma_epsilon(&inp);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn oxygen() {
        let inp = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");
        let oxy = get_oxygen(&inp);
        assert_eq!(oxy, 23);
    }

    #[test]
    fn co2() {
        let inp = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");
        let co2 = get_co2(&inp);
        assert_eq!(co2, 10);
    }
}

fn get_gamma_epsilon(inp: &Vec<String>) -> (u64, u64) {


    let v: Vec<char> = vec!['0'; inp[0].len()];
    let t:Vec<usize> = v.iter().enumerate().map(|(i, _)| inp.iter().fold(0, |acc, x| if x.chars().nth(i).unwrap()=='1' {acc+1} else {acc})).collect();
    let wx = inp.len()/2;

    let gamma:String   = t.iter().map(|&x| if x >= wx {'1'} else {'0'} ).collect::<Vec<char>>().iter().collect();
    let epsilon:String = t.iter().map(|&x| if x  > wx {'0'} else {'1'} ).collect::<Vec<char>>().iter().collect();

    (u64::from_str_radix(&gamma, 2).unwrap(), u64::from_str_radix(&epsilon, 2).unwrap())
}

fn get_oxygen(inp: &Vec<String>) -> u64 {
    let mut m_data: Vec<&String> = inp.iter().collect();
    for i in 0..inp[0].len() {
        let ones:usize = m_data.iter().fold(0, |acc, x| if x.chars().nth(i).unwrap()=='1' {acc+1} else {acc});
        let zeroes = m_data.len()-ones;
        let c = if ones >= zeroes {'1'} else {'0'};
        m_data = m_data.into_iter().filter(|x| x.chars().nth(i).unwrap() == c).collect();
        if m_data.len() == 1 {
            break;
        }
    }
    u64::from_str_radix(m_data[0], 2).unwrap()
}

fn get_co2(inp: &Vec<String>) -> u64 {
    let mut m_data: Vec<&String> = inp.iter().collect();
    for i in 0..inp[0].len() {
        let zeros:usize = m_data.iter().fold(0, |acc, x| if x.chars().nth(i).unwrap()=='0' {acc+1} else {acc});
        let ones = m_data.len()-zeros;
        let c = if ones >= zeros {'0'} else {'1'};
        m_data = m_data.into_iter().filter(|x| x.chars().nth(i).unwrap() == c).collect();
        if m_data.len() == 1 {
            break;
        }
    }
    u64::from_str_radix(m_data[0], 2).unwrap()
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 3");

    let inp = helpers::read_file_to_vec::<String>("inputs/day3.txt");
    let (gamma, epsilon) = get_gamma_epsilon(&inp);
    println!("Part1: {}", gamma*epsilon);

    let co2 = get_co2(&inp);
    let oxy = get_oxygen(&inp);
    println!("Part2: {}", oxy*co2);

    Ok(())
}
