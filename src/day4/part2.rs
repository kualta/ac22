use std::fs;

#[rustfmt::skip]
fn main() {
    let input = fs::read_to_string("src/day4/input.txt").unwrap();
    let mut amount = 0;
    let mut count = 0;

    input.lines().for_each(|line| {
        let mut nums = line.split(',')
            .flat_map(|range| range.split('-').collect::<Vec<_>>());

        let first_start = nums.next().unwrap().parse::<i32>().unwrap();
        let first_end = nums.next().unwrap().parse::<i32>().unwrap();
        let second_start = nums.next().unwrap().parse::<i32>().unwrap();
        let second_end = nums.next().unwrap().parse::<i32>().unwrap();

        if (first_start > second_end) || (first_end < second_start) {
            amount += 1;
        }
        count += 1;
    });

    println!("{}", count - amount);
}
