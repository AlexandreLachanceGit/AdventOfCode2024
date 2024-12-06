const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    None,
}

impl Direction {
    fn get_next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::None => Direction::None,
        }
    }

    fn get_move(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::None => (0, 0),
        }
    }
}

enum MoveResult {
    Loop,
    OutOfBounds,
    InProgress,
}

#[derive(Clone, Copy)]
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

    fn do_move(
        &mut self,
        map: &[Vec<char>],
        visited: &mut [Vec<Direction>],
        new_obstacle_pos: (usize, usize),
    ) -> MoveResult {
        let new_pos = (
            self.pos.0 as isize + self.dir.get_move().0,
            self.pos.1 as isize + self.dir.get_move().1,
        );
        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= map.len() as isize
            || new_pos.1 >= map[0].len() as isize
        {
            return MoveResult::OutOfBounds;
        }

        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);

        if visited[new_pos.0][new_pos.1] == self.dir {
            return MoveResult::Loop;
        }

        if map[new_pos.0][new_pos.1] == '#' || new_pos == new_obstacle_pos {
            self.dir = self.dir.get_next();
        } else {
            self.pos = new_pos;
            visited[self.pos.0][self.pos.1] = self.dir;
        }

        MoveResult::InProgress
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
    let guard = Guard::find(&map).unwrap();

    let mut count = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' || guard.pos == (i, j) {
                continue;
            }

            let mut visited = vec![vec![Direction::None; map[0].len()]; map.len()];
            visited[guard.pos.0][guard.pos.1] = Direction::Up;

            let mut guard = guard;

            loop {
                match guard.do_move(&map, &mut visited, (i, j)) {
                    MoveResult::Loop => {
                        count += 1;
                        break;
                    }
                    MoveResult::OutOfBounds => {
                        break;
                    }
                    MoveResult::InProgress => {}
                }
            }
        }
    }

    count
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
        assert_eq!(process(input), 6)
    }
}
