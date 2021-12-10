use std::io;

use crate::helpers;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupted_lines() {
        let test_strings:Vec<(&str, u32)> = vec![
            ("[({(<(())[]>[[{[]{<()<>>", 0),
            ("({([(<{}[<>[]}>{[]{[(<()>", 1197),
            ("[[<[([]))<([[{}[[()]]]", 3),
            ("<{([([[(<>()){}]>(<<{{", 25137),
            ("[{[{({}]{}}([{[{{{}}([]", 57),
        ];
        for (x, res) in test_strings {
            assert_eq!(find_corrupted(x), res);
        }
    }

    #[test]
    fn test_fix_lines() {
        let test_strings:Vec<(&str, u64)> = vec![
            ("[({(<(())[]>[[{[]{<()<>>", 288957),
            ("[(()[<>])]({[<{<<[]>>(", 5566),
            ("(((({<>}<{<{<>}{[]{[]{}", 1480781),
            ("{<[[]]>}<{[{[{[]{()[[[]", 995444),
            ("<{([{{}}[<[[[<>{}]]]>[]]", 294),
        ];
        for (x, res) in test_strings {
            assert_eq!(fix_lines(x), res);
        }

    }
    #[test]
    fn complete() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day10_test.txt");
        let sum:u32 = lines.iter().map(|x| find_corrupted(x)).sum();
        assert_eq!(sum, 26397);

        let mut fixeds:Vec<u64> = lines.iter().map(|x| fix_lines(x)).filter(|x|x>&0).collect();
        fixeds.sort();
        assert_eq!(fixeds[fixeds.len()/2], 288957);
    }
}


fn find_corrupted(inp: &str) -> u32 {
    let mut expected:Vec<char> = Vec::new();
    for x in inp.chars() {
        match x {
            '(' => expected.push(')'),
            '[' => expected.push(']'),
            '{' => expected.push('}'),
            '<' => expected.push('>'),
            _ => {
                let l = expected.pop().unwrap();
                if l != x {
                    return match x {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                         _  => 0,
                    };
                }
            }
        }
    }
    0
}

fn fix_lines(inp: &str) -> u64 {
    let mut expected:Vec<char> = Vec::new();
    for x in inp.chars() {
        match x {
            '(' => expected.push(')'),
            '[' => expected.push(']'),
            '{' => expected.push('}'),
            '<' => expected.push('>'),
            _ => {
                let l = expected.pop().unwrap();
                if l != x {
                    return 0;
                }
            }
        }
    }

    expected.into_iter().rev().fold(0, |sum, x| {
        sum * 5 + match x {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
             _  => 0,
        }
    })
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 10");

    let lines = helpers::read_file_to_vec::<String>("inputs/day10.txt");
    let sum:u32 = lines.iter().map(|x| find_corrupted(x)).sum();
    println!("Part1: {}", sum);

    let mut fixeds:Vec<u64> = lines.iter().map(|x| fix_lines(x)).filter(|x|x>&0).collect();
    fixeds.sort();
    println!("Part2: {:?}", fixeds[fixeds.len()/2]);

    Ok(())
}
