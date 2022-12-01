use std::fs;

fn main() {
    let input = fs::read_to_string("src/day1/input.txt").unwrap();

    let mut elves: Vec<i32> = vec![];
    let mut current = 0;

    input.lines().for_each(|line| match line {
        "" => {
            elves.push(current);
            current = 0;
        }
        _ => current += line.parse::<i32>().unwrap(),
    });

    elves.sort_by(|a, b| b.cmp(a));

    println!("{:?}", elves[0..3].iter().sum::<i32>());
}
