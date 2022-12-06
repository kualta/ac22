use std::fs;

fn main() {
    let input = fs::read_to_string("src/day6/input.txt").unwrap();
    let marker_length = 14;

    let (i, _) = input
        .as_bytes()
        .windows(marker_length)
        .enumerate()
        .find(|(_, w)| {
            let mut bytes = w.iter().collect::<Vec<_>>();
            bytes.sort();
            bytes.dedup();
            bytes.len() == marker_length
        })
        .unwrap();

    println!("{}", i + marker_length);
}
