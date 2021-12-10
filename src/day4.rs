use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn printboard<> (board: &Vec<(&u64, bool)>) {
    for (i, (&x, b)) in board.iter().enumerate() {
        if i > 0 && i % 5 == 0 {println!("");}
        if *b {
            print!("\t({})", x);
        } else {
            print!("\t {} ", x);
        }
    }
    println!("");
}

fn checkboard<> (board: &Vec<(&u64, bool)>) -> bool {

    // Check if rows are winning, we take chunks of 5 == one row
    let rows = board.chunks(5).map(|row| row.iter().filter( |(_v, k)| k==&true).count()).collect::<Vec<usize>>();
    // Then we check if some of theese sums up to 5
    if rows.iter().max() == Some(&5_usize) {
        return true
    }

    for i in 0..5 {
        let c1:bool = board.iter().skip(i).step_by(5).take(5).filter(|(_v,k)| k==&true).count() >= 5;
        if c1 { return true }
    }

    false
}
pub fn run() -> io::Result<()> {
    println!("\n\nDay 4");

    let file = File::open("inputs/day4.txt").expect("Could not open");
    let reader = BufReader::new(file);
    let mut line_iter = reader.lines();

    let numbers_s = line_iter.next().unwrap()?;
    let numbers = numbers_s.split(",").into_iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let boards = line_iter.map(|x| match x {Ok(d) => d.parse::<String>().unwrap(), Err(e)=>panic!("{:?}", e)}).collect::<Vec<String>>();

    // one empty line before each
    let ch:Vec<_> = boards.chunks(6).collect::<Vec<&[String]>>().into_iter()
        .map(|x| 
            x.iter().skip(1)
                .map(|y| y.split_whitespace()
                    .map(|n|
                        n.parse::<u64>().unwrap()
                    ).collect::<Vec<u64>>()
                ).collect::<Vec<Vec<u64>>>()

        ).collect();

    let b1 = ch.iter().map(|c| c.iter().flat_map(|x| x).collect::<Vec<&u64>>()).collect::<Vec<Vec<&u64>>>();
    let mut b2 = b1.iter().map(|b| b.iter().map(|&x| (x, false)).collect::<Vec<(&u64, bool)>>()).collect::<Vec<Vec<(&u64, bool)>>>();

    // List of boards not winning yet
    let mut boards_to_check = (0..b2.len()).collect::<Vec<usize>>();
    let b2minus1 = b2.len() - 1;

    for n in numbers {
        // Do this for all boards
        for i in boards_to_check.clone() {

            // Check and mark as drawn if found in board
            if let Some(x) = b2[i].iter().position(|&x| x == (&n, false)) {
                let (v, _) = b2[i][x];
                b2[i][x] = (v, true);

                // Check if board wins
                if checkboard(&b2[i]) {
                    match boards_to_check.iter().position(|&x| x == i as usize) {
                        Some(ins) => boards_to_check.remove(ins),
                        _ => 0_usize,
                    };

                    if boards_to_check.len() == b2minus1 {
                        let sum:u64 = b2[i].iter().filter( |(_v, k)| k==&false).map(|(&x, _)| x).collect::<Vec<u64>>().iter().sum();
                        println!("Part 1:\t{} * {} = {}", sum, n, sum*n);
                        printboard(&b2[i]);
                    } else if boards_to_check.len() == 0 {
                        let sum:u64 = b2[i].iter().filter( |(_v, k)| k==&false).map(|(&x, _)| x).collect::<Vec<u64>>().iter().sum();
                        println!("Part 2:\t{} * {} = {}", sum, n, sum*n);
                        printboard(&b2[i]);
                    }

                }
            }
        }
    }

    Ok(())
}
