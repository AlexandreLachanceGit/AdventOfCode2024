// Very hacky solution

use core::panic;
use std::{cmp::Ordering, collections::BinaryHeap, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
enum Position {
    Wall,
    Empty,
    Exit,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '#' => Position::Wall,
            '.' | 'S' => Position::Empty,
            'E' => Position::Exit,
            _ => panic!("invalid position char"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Reached {
    cost: usize,
    position: (usize, usize),
    dir: Direction,
}

impl PartialOrd for Reached {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for Reached {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

struct Maze {
    start: (usize, usize),
    maze: Vec<Vec<Position>>,
    visited: Vec<Vec<Option<(usize, (usize, usize))>>>,
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pos = Err(());
        for (i, l) in s.lines().enumerate() {
            if pos.is_ok() {
                break;
            }

            for (j, p) in l.chars().enumerate() {
                if p == 'S' {
                    pos = Ok((i, j));
                    break;
                }
            }
        }

        let maze: Vec<Vec<Position>> = s
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();

        Ok(Maze {
            start: pos?,
            visited: vec![vec![None; maze[0].len()]; maze.len()],
            maze,
        })
    }
}

impl Maze {
    fn get(&self, pos: (usize, usize)) -> Position {
        self.maze[pos.0][pos.1]
    }

    fn solve(&mut self) -> usize {
        self.visited[self.start.0][self.start.1] = Some((0, (0, 0)));

        let mut queue: BinaryHeap<Reached> = BinaryHeap::new();
        queue.push(Reached {
            cost: 0,
            position: self.start,
            dir: Direction::Right,
        });

        while let Some(curr) = queue.pop() {
            if matches!(self.get(curr.position), Position::Exit) {
                return curr.cost;
            }

            self.push_reached(curr.dir, curr.position, curr.cost + 1, &mut queue);
            let moved_l = self.push_reached(
                curr.dir.clockwise(),
                curr.position,
                curr.cost + 1000 + 1,
                &mut queue,
            );
            let moved_r = self.push_reached(
                curr.dir.counter_clockwise(),
                curr.position,
                curr.cost + 1000 + 1,
                &mut queue,
            );

            if moved_l || moved_r {
                self.visited[curr.position.0][curr.position.1] = Some((
                    self.visited[curr.position.0][curr.position.1].unwrap().0 + 1000,
                    self.visited[curr.position.0][curr.position.1].unwrap().1,
                ));
            }
        }

        panic!("no maze solution found")
    }

    fn push_reached(
        &mut self,
        dir: Direction,
        position: (usize, usize),
        new_cost: usize,
        queue: &mut BinaryHeap<Reached>,
    ) -> bool {
        let new_pos = increment_pos(position, dir.delta());
        if matches!(self.get(new_pos), Position::Empty | Position::Exit) {
            let visited_pos = self.visited[new_pos.0][new_pos.1];
            if (visited_pos.is_some() && new_cost < visited_pos.unwrap().0) || visited_pos.is_none()
            {
                self.visited[new_pos.0][new_pos.1] = Some((new_cost, position));
                queue.push(Reached {
                    cost: new_cost,
                    position: new_pos,
                    dir,
                });
                return true;
            }
        }
        false
    }

    fn count_tiles(&self, max_cost: usize) -> usize {
        fn helper(
            counted: &mut [Vec<bool>],
            visited: &[Vec<Option<(usize, (usize, usize))>>],
            start: (usize, usize),
        ) {
            let mut stack = vec![start];
            while let Some(curr) = stack.pop() {
                if curr == (visited.len() - 2, 1) || counted[curr.0][curr.1] {
                    continue;
                }
                counted[curr.0][curr.1] = true;

                let pre = visited[curr.0][curr.1].unwrap().1;
                stack.push(pre);

                Direction::DIRECTIONS.iter().for_each(|delta| {
                    let new_pos = increment_pos(curr, *delta);
                    if new_pos == pre
                        || counted[new_pos.0][new_pos.1]
                        || (visited[new_pos.0][new_pos.1].is_some()
                            && visited[new_pos.0][new_pos.1].unwrap().1 == curr)
                    {
                        return;
                    }

                    if visited[new_pos.0][new_pos.1].is_some()
                        && visited[new_pos.0][new_pos.1].unwrap().0
                            < visited[curr.0][curr.1].unwrap().0
                    {
                        stack.push(new_pos);
                    }
                });
            }
        }

        let mut counted = vec![vec![false; self.maze[0].len()]; self.maze.len()];
        let end = (1, self.maze[0].len() - 2);

        helper(&mut counted, &self.visited, end);

        self.get_paths(&counted, max_cost)
            .iter()
            .flatten()
            .unique()
            .count()
    }

    fn get_paths(&self, counted: &[Vec<bool>], max_cost: usize) -> Vec<Vec<(usize, usize)>> {
        fn helper(
            maze: &Maze,
            dir: Direction,
            current: Vec<(usize, usize)>,
            cost: usize,
            counted: &[Vec<bool>],
            max_cost: usize,
        ) -> Vec<Vec<(usize, usize)>> {
            if matches!(maze.get(*current.last().unwrap()), Position::Exit) {
                vec![current]
            } else {
                let mut paths = vec![];

                for &(d, c) in &[
                    (dir, 1),
                    (dir.clockwise(), 1001),
                    (dir.counter_clockwise(), 1001),
                ] {
                    let new_pos = increment_pos(*current.last().unwrap(), d.delta());
                    if matches!(maze.get(new_pos), Position::Empty | Position::Exit)
                        && cost + c <= max_cost
                        && counted[new_pos.0][new_pos.1]
                        && !current.contains(&new_pos)
                    {
                        let mut new_path = current.clone();
                        new_path.push(new_pos);
                        paths.extend(helper(maze, d, new_path, cost + c, counted, max_cost));
                    }
                }

                paths
            }
        }

        helper(
            self,
            Direction::Right,
            vec![self.start],
            0,
            counted,
            max_cost,
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn increment_pos(pos: (usize, usize), delta: (i32, i32)) -> (usize, usize) {
    (
        (pos.0 as i32 + delta.0) as usize,
        (pos.1 as i32 + delta.1) as usize,
    )
}

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> usize {
    let mut maze = Maze::from_str(input).unwrap();

    let max_cost = maze.solve();

    maze.count_tiles(max_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        assert_eq!(process(input), 45)
    }

    #[test]
    fn example() {
        let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
        assert_eq!(process(input), 64)
    }
}
