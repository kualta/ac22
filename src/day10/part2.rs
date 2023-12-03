use std::fs;

fn main() {
    let input = fs::read_to_string("src/day10/input.txt").unwrap();

    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;

    print!("#");
    input.lines().for_each(|line| match line {
        _ => {
            let (operation, magnitude) = line.split_once(' ').unwrap_or(("noop", ""));
            let screen: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];

            match operation {
                "noop" => {
                    cycle += 1;
                    if (x - 1..=x + 1).contains(&(cycle % 40)) {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                    if ((cycle + 1) % 40) == 0 {
                        println!();
                    }
                }
                "addx" => {
                    cycle += 1;
                    if (x - 1..=x + 1).contains(&(cycle % 40)) {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                    if ((cycle + 1) % 40) == 0 {
                        println!();
                    }

                    cycle += 1;
                    x += magnitude.parse::<i32>().unwrap();
                    if (x - 1..=x + 1).contains(&(cycle % 40)) {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                    if ((cycle + 1) % 40) == 0 {
                        println!();
                    }
                }
                _ => unreachable!(),
            }
        }
    });
    println!("");
}
