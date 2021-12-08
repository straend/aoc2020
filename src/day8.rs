use std::io;

use crate::helpers;
use itertools::Itertools;
use std::collections::HashMap;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unique() {
        let str = "fgae cfgab fg bagce";
        assert_eq!(count_1478(str), 2);

        // lengths of 7 2 3
        let str = "fcgedb cgb dgebacf gc";
        assert_eq!(count_1478(str), 3);

    }

    #[test]
    fn all_seven() {
        // 1234
        let str = "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let t = str.split(" | ").collect::<Vec<&str>>();
        assert_eq!(get_seven_output(t[0], t[1]), 4315);

        // 56
        let str = "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef";
        let t = str.split(" | ").collect::<Vec<&str>>();
        assert_eq!(get_seven_output(t[0], t[1]), 1625);

        //  789
        let str = "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc";
        let t = str.split(" | ").collect::<Vec<&str>>();
        assert_eq!(get_seven_output(t[0], t[1]), 9781);
    }
}

fn count_1478(input: &str) -> u32 {
    // Counts the unique values
    input.split(" ").filter(|&x| match x.len() {
        2..=4 => true,
        7 => true,
        _=> false,
    }).collect::<Vec<&str>>().len() as u32
}

fn get_seven_output(input: &str, out: &str) -> u32 {
    let mut seven_mappings:HashMap<String, char> = HashMap::new();

    // Split digits, and sort them
    let digits = input.split(" ").map(|x| x.chars().sorted().collect::<Vec<char>>().iter().cloned().collect::<String>() ).collect::<Vec<String>>();
    let outvec = out.split(" ").map(|x| x.chars().sorted().collect::<Vec<char>>().iter().cloned().collect::<String>() ).collect::<Vec<String>>();

    // Take the easy ones (only ones with 1 possibility)
    let index_2 = digits.iter().position(|r| r.len()==2).unwrap();
    let index_3 = digits.iter().position(|r| r.len()==3).unwrap();
    let index_4 = digits.iter().position(|r| r.len()==4).unwrap();
    let index_7 = digits.iter().position(|r| r.len()==7).unwrap();

    seven_mappings.insert(digits[index_2].to_string(), '1');
    seven_mappings.insert(digits[index_3].to_string(), '7');
    seven_mappings.insert(digits[index_4].to_string(), '4');
    seven_mappings.insert(digits[index_7].to_string(), '8');

    // for every len 5
    // 2 are same as in len 2 we have number 3 displayed
    // 2 are same as in len 4 we have 5
    // last len 5 is 2
    for dd in digits.iter().filter(|x| x.len() == 5){
        let c:char = match dd.chars().filter(|&x| digits[index_3].contains(x) ).count() {
            3 => '3',
            _ =>  match dd.chars().filter(|&y| digits[index_4].contains(y)).count() {
                    2 => '2',
                    _ => '5',
        }};
        seven_mappings.insert(dd.to_string(), c);
    };

    // For those of len 6
    // 2 same as in len 3 == 6
    // 4 same as in 4 == 9
    for dd in digits.iter().filter(|x| x.len() == 6){
        let c:char = match dd.chars().filter(|&x| digits[index_3].contains(x) ).count() {
            2 => '6',
            _ => match dd.chars().filter(|&y| digits[index_4].contains(y)).count() {
                4 => '9',
                _ => '0',
            },
        };
        seven_mappings.insert(dd.to_string(), c);
    };

    let ooot = outvec.iter().map(|x| seven_mappings.get(x).unwrap()).collect::<Vec<&char>>();
    String::from_iter(ooot).parse::<u32>().unwrap()
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 7");
    let lines = helpers::read_file_to_vec::<String>("inputs/day8.txt");
    let x:Vec<Vec<&str>> = lines.iter().map(|x| x.split("|").collect::<Vec<&str>>()).collect();
    let out:Vec<&str> = x.iter().map(|x| x[1].trim()).collect();
    let inp:Vec<&str> = x.iter().map(|x| x[0].trim()).collect();

    let sum1:u32 = out.to_vec().into_iter().map(|x| count_1478  (x)).sum();
    println!("Part1 {}", sum1);

    let sum:u32 = inp.iter().zip(out).map(|(i, o)| get_seven_output(i, o)).sum();
    println!("Part2 {}", sum);

    Ok(())
}
