use std::fs;

fn main() {
    let input = fs::read_to_string("src/day10/input.txt").unwrap();

    let mut cycle = 0;
    let mut x = 1;
    let cycles = [20, 60, 100, 140, 180, 220];
    let mut sum = 0;

    input.lines().for_each(|line| match line {
        _ => {
            let (operation, magnitude) = line.split_once(' ').unwrap_or(("noop", ""));

            match operation {
                "noop" => {
                    cycle += 1;
                    if cycles.contains(&cycle) {
                        println!("{cycle} {x}");
                        sum += x * cycle
                    }
                }
                "addx" => {
                    cycle += 1;
                    if cycles.contains(&cycle) {
                        println!("{cycle} {x} ");
                        sum += x * cycle
                    }

                    cycle += 1;
                    if cycles.contains(&cycle) {
                        println!("{cycle} {x} ");
                        sum += x * cycle
                    }

                    x += magnitude.parse::<i32>().unwrap();
                }
                _ => unreachable!(),
            }
        }
    });
    println!("{sum}");
}
