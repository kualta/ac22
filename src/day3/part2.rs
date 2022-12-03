#![feature(iter_next_chunk)]
use std::fs;

fn main() {
    let input = fs::read_to_string("src/day3/input.txt").unwrap();
    let mut input = input.lines();

    let lowercase_offset = 96;
    let uppercase_offset = 38;
    let mut score = 0;

    while let Ok(value) = input.next_chunk::<3>().map(|lines| {
        let mut first = lines[0].chars().collect::<Vec<_>>();
        let mut second = lines[1].chars().collect::<Vec<_>>();
        let mut third = lines[2].chars().collect::<Vec<_>>();

        first.sort();
        first.dedup();
        second.sort();
        second.dedup();
        third.sort();
        third.dedup();

        first
            .iter()
            .map(|&char| {
                let mut value = 0;
                if second.contains(&char) && third.contains(&char) {
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
    }) {
        score += value
    }

    println!("{score}");
}
