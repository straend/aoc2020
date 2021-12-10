use std::io;

use crate::helpers;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_lowpoints() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day9_test.txt");
        let data:Vec<Vec<u32>> = lines.iter().map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();
        let lowpoints = get_lowpoints(&data);
        assert_eq!(lowpoints, vec![(0,1), (0,9), (2,2),(4,6) ]);
    }

    #[test]
    fn check_island_sizes() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day9_test.txt");
        let data:Vec<Vec<u32>> = lines.iter().map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();
        let lowpoints = get_lowpoints(&data);
        let mut checked: Vec<Vec<bool>> = vec![vec![false; data[0].len()]; data.len()];
        let values:Vec<u32> = lowpoints.into_iter().map(|(row, col)| check(row, col, &data, &mut checked)).collect::<Vec<u32>>().to_vec();
        assert_eq!(values, vec![3, 9, 14, 9]);
    }

}


fn get_lowpoints(data: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {

    let rows = data.len();
    let cols = data[0].len();
    let mut out:Vec<(usize,usize)> = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            // Which directions to check
            let l = col > 0;
            let r = col < cols-1;
            let u = row > 0;
            let d = row < rows-1;

            let val = data[row][col];

            // if we should not check one direction set it to true
            // else compare with that direction
            let left = !l || l && val < data[row][col-1];
            let rigth = !r || r && val < data[row][col+1];
            let up = !u || u && val < data[row-1][col];
            let down = !d || d && val < data[row+1][col];

            if left && rigth && down && up {
                out.push((row, col));
            }
        }
    }
    out
}

fn check(y:usize, x: usize, data: &Vec<Vec<u32>>, checked: &mut Vec<Vec<bool>>) -> u32 {
    if checked[y][x] || data[y][x] == 9 {
        return 0;
    }
    checked[y][x] = true;
    let mut out = 1;

    if y > 0 {
        out+= check(y-1, x, data, checked);
    }
    if y < data.len()-1 {
        out+= check(y+1, x, data, checked);
    }
    if x > 0 {
        out+= check(y, x-1, data, checked);
    }
    if x < data[0].len()-1 {
        out+= check(y, x+1, data, checked);
    }
    out
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 9");
    let lines = helpers::read_file_to_vec::<String>("inputs/day9.txt");

    let data:Vec<Vec<u32>> = lines.iter().map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();

    let lowpoints = get_lowpoints(&data);

    let sum_lowpoints: u32 = lowpoints.iter().map(|(x,y)| data[*x][*y]+1).collect::<Vec<u32>>().iter().sum();
    println!("Part1: {}", sum_lowpoints);

    let mut checked: Vec<Vec<bool>> = vec![vec![false; data[0].len()]; data.len()];
    let mut values:Vec<u32> = lowpoints.into_iter().map(|(row, col)| check(row, col, &data, &mut checked)).collect::<Vec<u32>>().to_vec();
    values.sort();

    let prod3:u32 = values.into_iter().rev().take(3).product();
    println!("Part2: {}", prod3);
    Ok(())
}
