use std::fs;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        ((dx * dx) + (dy * dy)).sqrt()
    }
}

fn main() {
    let input = fs::read_to_string("src/day9/input.txt").unwrap();

    let rows = 1000;
    let cols = 1000;

    let mut visited: Vec<Vec<i32>> = vec![vec![0; cols]; rows];

    let mut head = Point { x: 500, y: 500 };
    let mut tail = Point { x: 500, y: 500 };
    println!("{}", head.distance(&tail));

    input.lines().for_each(|line| match line {
        _ => {
            let (direction, magnitude) = line.split_once(' ').unwrap();
            println!("{direction} {magnitude}");
            for _ in 0..magnitude.parse::<i32>().unwrap() {
                match direction {
                    "U" => head.y += 1,
                    "D" => head.y -= 1,
                    "L" => head.x -= 1,
                    "R" => head.x += 1,
                    _ => unreachable!(),
                }

                if head.distance(&tail) > 1.5 {
                    tail.x += (head.x - tail.x).signum();
                    tail.y += (head.y - tail.y).signum();
                }

                println!("{head:?} {tail:?}");
                visited[tail.y as usize][tail.x as usize] = 1;
            }
        }
    });

    println!("{:?}", visited);
    println!("{}", visited.iter().flatten().filter(|x| **x > 0).count());
}
