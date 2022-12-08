use std::fs;

fn main() {
    let input = fs::read_to_string("src/day8/input.txt").unwrap();
    let mut amount = 0;

    let matrix = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    for y in 0..99_usize {
        for x in 0..99_usize {
            if !is_hidden(x, y, &matrix) {
                amount += 1;
            }
        }
    }

    println!("{amount}");
}

#[rustfmt::skip]
fn is_hidden(x: usize, y: usize, matrix: &[u32]) -> bool {
    let x_max = 99;

    let row_start = x_max*y;
    let row = &matrix[row_start..row_start+x_max];
    let col = &matrix.iter().skip(x).step_by(x_max).collect::<Vec<_>>();
    let current = row[x];

    let left =   row[..x].iter().rev().any(|&height| height >= current);
    let right =  row[x+1..].iter().any(|&height| height >= current);
    let top =    col[..y].iter().rev().any(|&&height| height >= current);
    let bottom = col[y+1..].iter().any(|&&height| height >= current);

    left && right && top && bottom
}
