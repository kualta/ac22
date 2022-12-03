use std::fs;

fn main() {
    let input = fs::read_to_string("src/day3/input.txt").unwrap();
    let lowercase_offset = 96;
    let uppercase_offset = 38;

    let score = input
        .lines()
        .map(|line| {
            let mut first = line[..line.len() / 2].chars().collect::<Vec<_>>();
            let mut second = line[line.len() / 2..].chars().collect::<Vec<_>>();

            first.sort();
            first.dedup();
            second.sort();
            second.dedup();

            first
                .iter()
                .map(|&char| {
                    let mut value = 0;
                    if second.contains(&char) {
                        value = char as u32;
                        let offset = match char {
                            c if c.is_lowercase() => lowercase_offset,
                            c if c.is_uppercase() => uppercase_offset,
                            _ => unreachable!(),
                        };
                        value -= offset;
                    };
                    value
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{score}");
}
