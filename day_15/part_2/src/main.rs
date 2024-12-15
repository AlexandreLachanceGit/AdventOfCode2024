// very convoluted solution

use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

const GPS_MULTIPLIER: usize = 100;

#[derive(Debug, Clone, Copy)]
enum Position {
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '#' => Position::Wall,
            '.' | '@' => Position::Empty,
            '[' => Position::BoxLeft,
            ']' => Position::BoxRight,
            _ => panic!("invalid position char"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
                    robot_pos = Ok((i, j * 2));
                    break;
                }
            }
        }

        let map: Vec<Vec<Position>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .flat_map(|c| match c {
                        '#' | '.' => [c, c],
                        '@' => ['.', '.'],
                        'O' => ['[', ']'],
                        _ => panic!("invalid map char"),
                    })
                    .map(|c| c.into())
                    .collect()
            })
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
            Position::BoxLeft | Position::BoxRight => {
                if self.move_boxes(new_pos, robot_move) {
                    self.robot_pos = new_pos;
                }
            }
            Position::Wall => {}
        }
    }

    fn box_can_move(&mut self, pos: (usize, usize), box_move: Move) -> bool {
        let delta = box_move.delta();
        let (pos_l, pos_r) = match self.map[pos.0][pos.1] {
            Position::BoxLeft => (pos, (pos.0, pos.1 + 1)),
            Position::BoxRight => ((pos.0, pos.1 - 1), pos),
            _ => panic!("no box at position"),
        };
        let (new_pos_l, new_pos_r) = (increment_pos(pos_l, delta), increment_pos(pos_r, delta));

        match box_move {
            Move::Up | Move::Down => match (
                self.map[new_pos_l.0][new_pos_l.1],
                self.map[new_pos_r.0][new_pos_r.1],
            ) {
                (Position::Wall, _) | (_, Position::Wall) => false,
                (Position::Empty, Position::Empty) => true,
                (Position::BoxLeft, Position::BoxRight) | (Position::BoxRight, Position::Empty) => {
                    self.box_can_move(new_pos_l, box_move)
                }
                (Position::Empty, Position::BoxLeft) => self.box_can_move(new_pos_r, box_move),
                (Position::BoxRight, Position::BoxLeft) => {
                    self.box_can_move(new_pos_l, box_move) && self.box_can_move(new_pos_r, box_move)
                }
                _ => panic!("invalid map"),
            },
            Move::Left | Move::Right => match (
                self.map[new_pos_l.0][new_pos_l.1],
                self.map[new_pos_r.0][new_pos_r.1],
            ) {
                (Position::Wall, _) | (_, Position::Wall) => false,
                (Position::Empty, Position::BoxLeft) => true,
                (Position::BoxRight, Position::Empty) => true,
                (Position::BoxRight, Position::BoxLeft) => {
                    if matches!(box_move, Move::Left) {
                        self.box_can_move(new_pos_l, box_move)
                    } else {
                        self.box_can_move(new_pos_r, box_move)
                    }
                }
                _ => panic!("invalid map"),
            },
        }
    }

    fn move_boxes(&mut self, pos: (usize, usize), box_move: Move) -> bool {
        // Check first if box be moved, then move
        let can_move = self.box_can_move(pos, box_move);

        if can_move {
            self.move_box(pos, box_move);
        }

        can_move
    }

    fn move_box(&mut self, pos: (usize, usize), box_move: Move) {
        let delta = box_move.delta();
        let (pos_l, pos_r) = match self.map[pos.0][pos.1] {
            Position::BoxLeft => (pos, (pos.0, pos.1 + 1)),
            Position::BoxRight => ((pos.0, pos.1 - 1), pos),
            _ => return,
        };
        let (new_pos_l, new_pos_r) = (increment_pos(pos_l, delta), increment_pos(pos_r, delta));

        match box_move {
            Move::Up | Move::Down => match (
                self.map[new_pos_l.0][new_pos_l.1],
                self.map[new_pos_r.0][new_pos_r.1],
            ) {
                (Position::BoxLeft, Position::BoxRight) | (Position::BoxRight, Position::Empty) => {
                    self.move_box(new_pos_l, box_move)
                }
                (Position::Empty, Position::BoxLeft) => self.move_box(new_pos_r, box_move),
                (Position::BoxRight, Position::BoxLeft) => {
                    self.move_box(new_pos_l, box_move);
                    self.move_box(new_pos_r, box_move);
                }
                _ => {}
            },
            Move::Left | Move::Right => {
                if let (Position::BoxRight, Position::BoxLeft) = (
                    self.map[new_pos_l.0][new_pos_l.1],
                    self.map[new_pos_r.0][new_pos_r.1],
                ) {
                    if matches!(box_move, Move::Left) {
                        self.move_box(increment_pos(new_pos_l, delta), box_move)
                    } else {
                        self.move_box(increment_pos(new_pos_r, delta), box_move)
                    }
                }
            }
        }

        match box_move {
            Move::Up | Move::Down => {
                self.map[pos_l.0][pos_l.1] = Position::Empty;
                self.map[pos_r.0][pos_r.1] = Position::Empty;
                self.map[new_pos_l.0][new_pos_l.1] = Position::BoxLeft;
                self.map[new_pos_r.0][new_pos_r.1] = Position::BoxRight;
            }
            Move::Left => {
                self.map[pos_r.0][pos_r.1] = Position::Empty;
                self.map[new_pos_l.0][new_pos_l.1] = Position::BoxLeft;
                self.map[new_pos_r.0][new_pos_r.1] = Position::BoxRight;
            }
            Move::Right => {
                self.map[pos_l.0][pos_l.1] = Position::Empty;
                self.map[new_pos_l.0][new_pos_l.1] = Position::BoxLeft;
                self.map[new_pos_r.0][new_pos_r.1] = Position::BoxRight;
            }
        }
    }

    fn sum_box_gps_coordinates(&self) -> usize {
        let mut total = 0;
        for (i, l) in self.map.iter().enumerate() {
            for (j, p) in l.iter().enumerate() {
                if matches!(p, Position::BoxLeft) {
                    total += GPS_MULTIPLIER * i + j;
                }
            }
        }
        total
    }

    fn print(&self) {
        for (i, l) in self.map.iter().enumerate() {
            for (j, p) in l.iter().enumerate() {
                if (i, j) == self.robot_pos {
                    print!("@");
                } else {
                    print!(
                        "{}",
                        match p {
                            Position::Wall => '#',
                            Position::Empty => '.',
                            Position::BoxLeft => '[',
                            Position::BoxRight => ']',
                        }
                    )
                }
            }
            println!();
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
    let mut sections = input.split("\n\n");

    let map_str = sections.next().unwrap();
    let mut map = Map::from_str(map_str).unwrap();

    let moves_str = sections.next().unwrap();
    moves_str
        .chars()
        .filter(|c| *c != '\n')
        .map(Move::from)
        .for_each(|m| map.move_robot(m));

    map.print();

    map.sum_box_gps_coordinates()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(process(input), 9021)
    }
}
