use core::panic;
use std::{cmp::Ordering, collections::BinaryHeap, str::FromStr};

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
    parent: Option<(usize, usize)>,
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
    pos: (usize, usize),
    maze: Vec<Vec<Position>>,
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

        let map: Vec<Vec<Position>> = s
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();

        Ok(Maze {
            pos: pos?,
            maze: map,
        })
    }
}

impl Maze {
    fn get(&self, pos: (usize, usize)) -> Position {
        self.maze[pos.0][pos.1]
    }

    fn solve(&self) -> usize {
        let mut visited: Vec<Vec<Option<usize>>> =
            vec![vec![None; self.maze[0].len()]; self.maze.len()];
        visited[self.pos.0][self.pos.1] = Some(0);

        let mut queue: BinaryHeap<Reached> = BinaryHeap::new();
        queue.push(Reached {
            cost: 0,
            position: self.pos,
            dir: Direction::Right,
            parent: None,
        });

        while let Some(curr) = queue.pop() {
            if matches!(self.get(curr.position), Position::Exit) {
                return curr.cost;
            }

            self.push_reached(
                curr.dir,
                curr.position,
                curr.cost + 1,
                &mut visited,
                &mut queue,
            );
            let moved_l = self.push_reached(
                curr.dir.clockwise(),
                curr.position,
                curr.cost + 1000 + 1,
                &mut visited,
                &mut queue,
            );
            let moved_r = self.push_reached(
                curr.dir.counter_clockwise(),
                curr.position,
                curr.cost + 1000 + 1,
                &mut visited,
                &mut queue,
            );

            if moved_l || moved_r {
                visited[curr.position.0][curr.position.1] =
                    Some(visited[curr.position.0][curr.position.1].unwrap() + 1000);
            }
        }

        panic!("no maze solution found")
    }

    fn push_reached(
        &self,
        dir: Direction,
        position: (usize, usize),
        new_cost: usize,
        visited: &mut [Vec<Option<usize>>],
        queue: &mut BinaryHeap<Reached>,
    ) -> bool {
        let new_pos = increment_pos(position, dir.delta());
        if matches!(self.get(new_pos), Position::Empty | Position::Exit) {
            let visited_pos = visited[new_pos.0][new_pos.1];
            if (visited_pos.is_some() && new_cost < visited_pos.unwrap()) || visited_pos.is_none() {
                visited[new_pos.0][new_pos.1] = Some(new_cost);
                queue.push(Reached {
                    cost: new_cost,
                    position: new_pos,
                    dir,
                    parent: Some(position),
                });
                return true;
            }
        }
        false
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
    let maze = Maze::from_str(input).unwrap();

    maze.solve()
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
        assert_eq!(process(input), 7036)
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
        assert_eq!(process(input), 11048)
    }

    #[test]
    fn other() {
        let input = r#"##########
#.......E#
#.##.#####
#..#.....#
##.#####.#
#S.......#
##########"#;
        assert_eq!(process(input), 4013)
    }
}
