use core::panic;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

const GPS_MULTIPLIER: usize = 100;

#[derive(Debug, Clone, Copy)]
enum Position {
    Wall,
    Empty,
    Box,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '#' => Position::Wall,
            '.' | '@' => Position::Empty,
            'O' => Position::Box,
            _ => panic!("invalid position char"),
        }
    }
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("invalid move char"),
        }
    }
}

impl Move {
    fn delta(&self) -> (i32, i32) {
        match self {
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            Move::Left => (0, -1),
            Move::Right => (0, 1),
        }
    }
}

struct Map {
    robot_pos: (usize, usize),
    map: Vec<Vec<Position>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robot_pos = Err(());
        for (i, l) in s.lines().enumerate() {
            if robot_pos.is_ok() {
                break;
            }

            for (j, p) in l.chars().enumerate() {
                if p == '@' {
                    robot_pos = Ok((i, j));
                    break;
                }
            }
        }

        let map: Vec<Vec<Position>> = s
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();

        Ok(Map {
            robot_pos: robot_pos?,
            map,
        })
    }
}

impl Map {
    fn move_robot(&mut self, robot_move: Move) {
        let delta = robot_move.delta();
        let new_pos = increment_pos(self.robot_pos, delta);

        match self.map[new_pos.0][new_pos.1] {
            Position::Empty => self.robot_pos = new_pos,
            Position::Box => {
                if self.move_box(new_pos, delta) {
                    self.robot_pos = new_pos;
                }
            }
            Position::Wall => {}
        }
    }

    fn move_box(&mut self, pos: (usize, usize), delta: (i32, i32)) -> bool {
        let new_pos = increment_pos(pos, delta);

        match self.map[new_pos.0][new_pos.1] {
            Position::Wall => false,
            Position::Empty => {
                self.map[pos.0][pos.1] = Position::Empty;
                self.map[new_pos.0][new_pos.1] = Position::Box;
                true
            }
            Position::Box => {
                if self.move_box(new_pos, delta) {
                    self.map[pos.0][pos.1] = Position::Empty;
                    self.map[new_pos.0][new_pos.1] = Position::Box;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn sum_box_gps_coordinates(&self) -> usize {
        let mut total = 0;
        for (i, l) in self.map.iter().enumerate() {
            for (j, p) in l.iter().enumerate() {
                if matches!(p, Position::Box) {
                    total += GPS_MULTIPLIER * i + j;
                }
            }
        }
        total
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
    let mut sections = input.split("\n\n");

    let map_str = sections.next().unwrap();
    let mut map = Map::from_str(map_str).unwrap();

    let moves_str = sections.next().unwrap();
    moves_str
        .chars()
        .filter(|c| *c != '\n')
        .map(Move::from)
        .for_each(|m| map.move_robot(m));

    map.sum_box_gps_coordinates()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        assert_eq!(process(input), 2028)
    }

    #[test]
    fn example() {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        assert_eq!(process(input), 10092)
    }
}
