use core::panic;
use std::{cmp::Ordering, collections::BinaryHeap, usize};

const INPUT: &str = include_str!("../input.txt");

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug, Eq, PartialEq)]
struct Space {
    h_cost: usize,
    cost: usize,
    pos: (usize, usize),
}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.h_cost.cmp(&self.h_cost))
    }
}

impl Ord for Space {
    fn cmp(&self, other: &Self) -> Ordering {
        other.h_cost.cmp(&self.h_cost)
    }
}

fn increment_pos(pos: (usize, usize), delta: (i32, i32), size: usize) -> Option<(usize, usize)> {
    let new_pos = ((pos.0 as i32 + delta.0), (pos.1 as i32 + delta.1));
    if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= size as i32 || new_pos.1 >= size as i32 {
        None
    } else {
        Some((new_pos.0 as usize, new_pos.1 as usize))
    }
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn path_exits(end: (usize, usize), corrupted: &[Vec<bool>], size: usize) -> bool {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; size]; size];
    let mut queue: BinaryHeap<Space> = BinaryHeap::new();
    queue.push(Space {
        h_cost: manhattan_distance((0, 0), end),
        cost: 0,
        pos: (0, 0),
    });

    while let Some(curr) = queue.pop() {
        if curr.pos == end {
            return true;
        }
        visited[curr.pos.1][curr.pos.0] = true;

        DIRECTIONS.iter().for_each(|delta| {
            if let Some(new_pos) = increment_pos(curr.pos, *delta, size) {
                if !visited[new_pos.1][new_pos.0] && !corrupted[new_pos.1][new_pos.0] {
                    queue.push(Space {
                        h_cost: curr.cost + 1 + manhattan_distance(new_pos, end),
                        cost: curr.cost + 1,
                        pos: new_pos,
                    });
                }
            }
        })
    }

    false
}

fn main() {
    println!("Answer: {}", process(INPUT, 71));
}

fn process(input: &str, size: usize) -> String {
    let end = (size - 1, size - 1);

    let mut corrupted: Vec<Vec<bool>> = vec![vec![false; size]; size];
    let bytes = input
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    if let Some(byte) = bytes.iter().find(|pos| {
        corrupted[pos.1][pos.0] = true;
        !path_exits(end, &corrupted, size)
    }) {
        format!("{},{}", byte.0, byte.1).to_string()
    } else {
        panic!("no bytes block path")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
        assert_eq!(process(input, 7), "6,1")
    }
}
