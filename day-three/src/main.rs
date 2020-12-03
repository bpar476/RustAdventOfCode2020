use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct SlopeTraversal {
    right: usize,
    down: usize,
}

fn traversal(right: usize, down: usize) -> SlopeTraversal {
    SlopeTraversal { right, down }
}

fn main() {
    let file = File::open("input.txt");
    let reader = BufReader::new(file.unwrap());

    let traversal_algorithms = [
        traversal(1, 1),
        traversal(3, 1),
        traversal(5, 1),
        traversal(7, 1),
        traversal(1, 2),
    ];

    let slope = reader
        .lines()
        .map(|line| -> Vec<bool> {
            line.unwrap()
                .chars()
                .map(|char| -> bool { char == '.' })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let repeat_len = slope[0].len();

    let mut multi_sum = 1;
    for algo in traversal_algorithms.iter() {
        let mut trees = 0;
        let mut horizontal_position = 0;
        for i in (0..slope.len() - algo.down).step_by(algo.down) {
            horizontal_position += algo.right;
            if !slope[i + algo.down][horizontal_position % repeat_len] {
                trees += 1;
            }
        }
        println!("Bumped into {} trees with slope {:?}", trees, algo);
        multi_sum *= trees;
    }

    println!("Multi sum is {}", multi_sum);
}
