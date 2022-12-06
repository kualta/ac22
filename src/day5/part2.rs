#![feature(iter_advance_by)]
use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("src/day5/input.txt").unwrap();
    let mut input = input.lines();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 10];

    // populate stacks
    input.by_ref().take(8).for_each(|line| {
        let line = &line[1..];
        line.chars().step_by(4).enumerate().for_each(|(i, c)| {
            if !c.is_whitespace() {
                stacks[i].push(c);
            }
        })
    });
    stacks.iter_mut().for_each(|stack| stack.reverse());

    // simulate rearragement
    input.skip(2).for_each(|line| {
        let mut values = line
            .split_whitespace()
            .filter_map(|str| str.parse::<usize>().ok());
        let amount = values.next().unwrap();
        let from = values.next().unwrap() - 1;
        let to = values.next().unwrap() - 1;

        let start = stacks[from].len() - amount;
        let mut crates = stacks[from].drain(start..).collect::<Vec<char>>();
        stacks[to].append(&mut crates)
    });

    // print the top crate in each stack
    stacks
        .iter()
        .for_each(|stack| print!("{}", stack.last().unwrap_or(&' ')));
}
