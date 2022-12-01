use std::fs;

fn main() {
    let input = fs::read_to_string("src/day1/input.txt").unwrap();

    let mut max = 0;
    let mut current = 0;

    input.lines().for_each(|line| match line {
        "" => {
            if current > max {
                max = current
            }
            current = 0
        }
        _ => current += line.parse::<i32>().unwrap(),
    });

    println!("{max}");
}
