const INPUT: &str = include_str!("../input.txt");

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn get_next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_move(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

struct Guard {
    pos: (usize, usize),
    dir: Direction,
}

impl Guard {
    fn find(map: &[Vec<char>]) -> Option<Guard> {
        for (i, row) in map.iter().enumerate() {
            if let Some(j) = row.iter().position(|x| *x == '^') {
                return Some(Guard {
                    pos: (i, j),
                    dir: Direction::Up,
                });
            }
        }

        None
    }

    /// Executes next move. Returns `true` when it goes out of map.
    fn do_move(&mut self, map: &[Vec<char>], visited: &mut [Vec<bool>]) -> bool {
        visited[self.pos.0][self.pos.1] = true;

        let new_pos = (
            self.pos.0 as isize + self.dir.get_move().0,
            self.pos.1 as isize + self.dir.get_move().1,
        );
        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= map.len() as isize
            || new_pos.1 >= map[0].len() as isize
        {
            return true;
        }

        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

        if map[new_pos.0][new_pos.1] == '#' {
            self.dir = self.dir.get_next();
        } else {
            self.pos = new_pos;
        }

        false
    }
}

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut guard = Guard::find(&map).unwrap();

    visited[guard.pos.0][guard.pos.1] = true;

    loop {
        if guard.do_move(&map, &mut visited) {
            break;
        }
    }

    visited
        .iter()
        .fold(0, |acc, row| acc + row.iter().filter(|x| **x).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(process(input), 41)
    }
}
