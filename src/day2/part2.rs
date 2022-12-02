use std::fs;

fn main() {
    let input = fs::read_to_string("src/day2/input.txt").unwrap();

    let score = input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            let player1 = line.next().unwrap();
            let outcome = line.next().unwrap();
            let player2 = shape_from_outcome(player1, outcome);
            shape_value(&player2) + round_score(player1, &player2)
        })
        .sum::<i32>();

    println!("{score}");
}

fn shape_from_outcome(p1: &str, outcome: &str) -> String {
    match p1 {
        "A" => match outcome {
            "X" => "Z",
            "Y" => "X",
            "Z" => "Y",
            _ => panic!(),
        },
        "B" => match outcome {
            "X" => "X",
            "Y" => "Y",
            "Z" => "Z",
            _ => panic!(),
        },
        "C" => match outcome {
            "X" => "Y",
            "Y" => "Z",
            "Z" => "X",
            _ => panic!(),
        },
        _ => panic!(),
    }
    .to_owned()
}

fn shape_value(shape: &str) -> i32 {
    match shape {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!(),
    }
}

fn round_score(p1: &str, p2: &str) -> i32 {
    let p1 = match p1 {
        "A" => "R",
        "B" => "P",
        "C" => "S",
        _ => panic!(),
    };

    let p2 = match p2 {
        "X" => "R",
        "Y" => "P",
        "Z" => "S",
        _ => panic!(),
    };

    match (p1, p2) {
        ("S", "R") => 6,
        ("P", "S") => 6,
        ("R", "P") => 6,
        ("R", "R") => 3,
        ("P", "P") => 3,
        ("S", "S") => 3,
        ("R", "S") => 0,
        ("S", "P") => 0,
        ("P", "R") => 0,
        _ => panic!(),
    }
}
