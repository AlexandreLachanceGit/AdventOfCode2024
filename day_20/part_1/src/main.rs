use core::panic;
use std::{collections::HashSet, str::FromStr};

const INPUT: &str = include_str!("../input.txt");

const CHEAT_TIME: usize = 2;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn left(&self) -> Direction {
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

#[derive(PartialEq, Eq, Hash, Debug)]
struct Cheat {
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Clone, Copy, Debug)]
enum Position {
    Wall,
    Track,
}

struct RaceTrack {
    map: Vec<Vec<Position>>,
    size: (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
    track: Vec<(usize, usize)>,
}

impl FromStr for RaceTrack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<_>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => Position::Wall,
                        '.' | 'E' | 'S' => Position::Track,
                        _ => panic!("invalid map string"),
                    })
                    .collect()
            })
            .collect();

        let mut start = (0, 0);
        let mut end = (0, 0);
        for (i, l) in s.lines().enumerate() {
            for (j, c) in l.chars().enumerate() {
                if c == 'S' {
                    start = (i, j);
                } else if c == 'E' {
                    end = (i, j);
                }
            }
        }

        Ok(RaceTrack {
            size: (map.len(), map[0].len()),
            map,
            start,
            end,
            track: vec![],
        })
    }
}

impl RaceTrack {
    fn mesure_track(&mut self) -> Vec<Vec<Option<usize>>> {
        let mut pos = self.start;
        let mut cost = 0;
        let mut visited = vec![vec![None; self.map[0].len()]; self.map.len()];

        while pos != self.end {
            self.track.push(pos);
            visited[pos.0][pos.1] = Some(cost);

            for dir in Direction::DIRECTIONS {
                let new_pos = self.increment_pos(pos, dir.delta()).unwrap();
                if matches!(self.map[new_pos.0][new_pos.1], Position::Track)
                    && visited[new_pos.0][new_pos.1].is_none()
                {
                    pos = new_pos;
                    break;
                }
            }

            cost += 1;
        }
        visited[self.end.0][self.end.1] = Some(cost);
        self.track.push(pos);

        visited
    }

    fn count_cheats(&mut self, threshold: usize) -> usize {
        let visited = self.mesure_track();

        let mut cheats: HashSet<Cheat> = HashSet::new();
        for pos in &self.track {
            Direction::DIRECTIONS
                .iter()
                .filter(|dir| {
                    if let Some(new_pos) = self.increment_pos(*pos, dir.delta()) {
                        matches!(self.get(new_pos), Position::Wall)
                    } else {
                        false
                    }
                })
                .flat_map(|dir| self.cheat(*pos, *dir, CHEAT_TIME))
                .for_each(|end| {
                    cheats.insert(Cheat { start: *pos, end });
                });
        }

        cheats
            .iter()
            .map(|c| self.saved(&visited, c))
            .filter(|saved| *saved >= threshold)
            .count()
    }

    fn cheat(&self, pos: (usize, usize), dir: Direction, ttl: usize) -> Vec<(usize, usize)> {
        if let Some(pos) = self.increment_pos(pos, dir.delta()) {
            match (ttl, self.get(pos)) {
                (1, Position::Track) => vec![pos],
                (1, Position::Wall) => vec![],
                (_, Position::Wall) => [dir, dir.left(), dir.right()]
                    .iter()
                    .flat_map(|dir| self.cheat(pos, *dir, ttl - 1))
                    .collect(),
                (_, Position::Track) => {
                    let mut children: Vec<_> = [dir, dir.left(), dir.right()]
                        .iter()
                        .flat_map(|dir| self.cheat(pos, *dir, ttl - 1))
                        .collect();
                    children.push(pos);
                    children
                }
            }
        } else {
            vec![]
        }
    }

    fn saved(&self, visited: &[Vec<Option<usize>>], cheat: &Cheat) -> usize {
        let cost_start = visited[cheat.start.0][cheat.start.1].unwrap();
        let cost_end = visited[cheat.end.0][cheat.end.1].unwrap();

        cost_end.saturating_sub(cost_start + CHEAT_TIME)
    }

    fn get(&self, pos: (usize, usize)) -> Position {
        self.map[pos.0][pos.1]
    }

    fn increment_pos(&self, pos: (usize, usize), delta: (i32, i32)) -> Option<(usize, usize)> {
        let new_pos = (
            (pos.0 as i32).checked_add(delta.0)? as usize,
            (pos.1 as i32).checked_add(delta.1)? as usize,
        );

        if new_pos.0 >= self.size.0 || new_pos.1 >= self.size.1 {
            None
        } else {
            Some(new_pos)
        }
    }
}

fn main() {
    println!("Answer: {}", process(INPUT, 100));
}

fn process(input: &str, threshold: usize) -> usize {
    let mut track = RaceTrack::from_str(input).unwrap();
    track.count_cheats(threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        assert_eq!(process(input, 12), 8)
    }
}
