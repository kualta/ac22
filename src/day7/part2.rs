use std::{cell::RefCell, fs, rc::Rc};

#[derive(Default)]
struct Node {
    size: u32,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn print(&self, indent: usize) {
        if indent == 4 {
            println!("[{}]", self.size);
        }
        println!("{}", self.size);
        self.children
            .iter()
            .for_each(|child| child.as_ref().borrow().print(indent + 2));
    }
}

fn main() {
    let input = fs::read_to_string("src/day7/input.txt").unwrap();

    let root = Rc::new(RefCell::new(Node::default()));
    let mut current = Rc::clone(&root);
    let space_total = 70_000_000;
    let space_needed = 30_000_000;
    let mut dir_sizes: Vec<u32> = vec![];
    let mut space_used = 0;

    input.lines().for_each(|line| {
        let symbols = line.split_whitespace().collect::<Vec<_>>();
        if symbols.is_empty() {
            while current.borrow().parent.as_ref().is_some() {
                go_parent_dir(&mut current, &mut space_used, &mut dir_sizes);
            }
            return;
        }

        match symbols[0] {
            "$" => {
                if let "cd" = symbols[1] {
                    if let ".." = symbols[2] {
                        go_parent_dir(&mut current, &mut space_used, &mut dir_sizes);
                    } else {
                        let child = Rc::new(RefCell::new(Node {
                            parent: Some(Rc::clone(&current)),
                            ..Default::default()
                        }));
                        current.as_ref().borrow_mut().children.push(child.clone());
                        current = child;
                    }
                }
            }
            x if x.parse::<u32>().is_ok() => {
                let num = x.parse::<u32>().unwrap();
                current.as_ref().borrow_mut().size += num;
            }
            "dir" => (),
            _ => unreachable!(),
        };
    });
    root.borrow().print(0);

    // dir_sizes.sort();
    let space_left = space_total - space_used;
    let size = dir_sizes
        .iter()
        .find(|&&size| space_left + size >= space_needed)
        .unwrap();

    println!("{size}");
}

fn go_parent_dir(current: &mut Rc<RefCell<Node>>, sum: &mut u32, dir_sizes: &mut Vec<u32>) {
    let mut last_pass = true;
    let size = current.as_ref().borrow_mut().size;
    let child_size = current
        .as_ref()
        .borrow_mut()
        .children
        .iter()
        .map(|child| {
            let size = child.as_ref().borrow().size;
            if size == 0 {
                last_pass = false;
            }
            size
        })
        .sum::<u32>();

    let total_size = size + child_size;
    if last_pass {
        *sum += total_size;
        println!("{} = {} + {}", *sum, size, child_size);
        dir_sizes.push(total_size);
    }
    *current = Rc::clone(current.clone().borrow().parent.as_ref().unwrap());
}
