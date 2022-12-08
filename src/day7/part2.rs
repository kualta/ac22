use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("src/day7/input.txt").unwrap();
    let space_total = 70_000_000;
    let space_needed = 30_000_000;

    let mut tree: BTreeMap<Vec<&str>, u32> = BTreeMap::new();
    let mut context: Vec<&str> = vec![];

    // parse the tree
    input.lines().for_each(|line| {
        let symbols = line.split_whitespace().collect::<Vec<_>>();
        execute_cmd(symbols, &mut tree, &mut context);
    });

    // unwind the remaining stack
    (0..context.len()).for_each(|_| {
        let command = vec!["$", "cd", ".."];
        execute_cmd(command, &mut tree, &mut context)
    });

    let space_used = tree[&vec!["/"]];
    let space_left = space_total - space_used;

    let mut dir_sizes = Vec::from_iter(tree.values());
    dir_sizes.sort();

    let size = dir_sizes
        .iter()
        .find(|&&size| space_left + size >= space_needed)
        .unwrap();

    println!("{size}");
}

fn execute_cmd<'a>(
    command: Vec<&'a str>,
    tree: &mut BTreeMap<Vec<&'a str>, u32>,
    context: &mut Vec<&'a str>,
) {
    println!("{command:?}");
    match command[0] {
        "$" => {
            if let "cd" = command[1] {
                if let ".." = command[2] {
                    let self_size = tree[context];
                    println!("{:?} - {}", context, self_size);
                    context.pop();
                    tree.entry(context.clone())
                        .and_modify(|size| *size += self_size);
                } else {
                    context.push(command[2]);
                    tree.entry(context.clone()).or_insert(0);
                }
            }
        }
        x if x.parse::<u32>().is_ok() => {
            let num = x.parse::<u32>().unwrap();
            tree.entry(context.clone())
                .and_modify(|size| *size += num)
                .or_insert(num);
        }
        "dir" => (),
        _ => unreachable!(),
    };
}
