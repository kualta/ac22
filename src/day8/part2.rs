use std::fs;

fn main() {
    let input = fs::read_to_string("src/day8/input.txt").unwrap();
    let mut max = 0;

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
            let score = get_score(x, y, &matrix);
            if max < score {
                max = score
            }
        }
    }

    println!("{max}");
}

#[rustfmt::skip]
fn get_score(x: usize, y: usize, matrix: &[u32]) -> usize {
    let x_max = 99;

    let row_start = x_max*y;
    let row = &matrix[row_start..row_start+x_max];
    let col = &matrix.iter().skip(x).step_by(x_max).collect::<Vec<_>>();
    let current = row[x];
    let mut side_scores = vec![0; 4];

    row[..x].iter().rev().find_map(|&height| {
        side_scores[0] += 1;
        (height >= current).then_some(height)
    });
    row[x+1..].iter().find_map(|&height| {
        side_scores[1] += 1;
        (height >= current).then_some(height)
    });
    col[..y].iter().rev().find_map(|&&height| {
        side_scores[2] += 1;
        (height >= current).then_some(height)
    });
    col[y+1..].iter().find_map(|&&height| {
        side_scores[3] += 1;
        (height >= current).then_some(height)
    });

    side_scores.iter().product()
}
