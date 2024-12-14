use std::collections::HashMap;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

const MAP_SIZE: (i32, i32) = (101, 103);
// const MAP_SIZE: (i32, i32) = (11, 7); For test
const NB_SECONDS: i32 = 100;

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl TryFrom<(i32, i32)> for Quadrant {
    type Error = ();

    fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
        match value {
            (x, y) if x < MAP_SIZE.0 / 2 && y < MAP_SIZE.1 / 2 => Ok(Quadrant::TopLeft),
            (x, y) if x < MAP_SIZE.0 / 2 && y > MAP_SIZE.1 / 2 => Ok(Quadrant::BottomLeft),
            (x, y) if x > MAP_SIZE.0 / 2 && y < MAP_SIZE.1 / 2 => Ok(Quadrant::TopRight),
            (x, y) if x > MAP_SIZE.0 / 2 && y > MAP_SIZE.1 / 2 => Ok(Quadrant::BottomRight),
            _ => Err(()),
        }
    }
}

impl Robot {
    fn get_final_pos(&self) -> (i32, i32) {
        let pos = (
            (NB_SECONDS * self.vel.0 + self.pos.0) % MAP_SIZE.0,
            (NB_SECONDS * self.vel.1 + self.pos.1) % MAP_SIZE.1,
        );

        (
            if pos.0 < 0 { pos.0 + MAP_SIZE.0 } else { pos.0 },
            if pos.1 < 0 { pos.1 + MAP_SIZE.1 } else { pos.1 },
        )
    }
}

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> i32 {
    let mut quadrants = HashMap::from([
        (Quadrant::TopLeft, 0),
        (Quadrant::TopRight, 0),
        (Quadrant::BottomLeft, 0),
        (Quadrant::BottomRight, 0),
    ]);

    parse_robots(input).iter().for_each(|r| {
        let pos = r.get_final_pos();
        if let Ok(quad) = &pos.try_into() {
            *quadrants.get_mut(quad).unwrap() += 1
        }
    });

    quadrants.values().product()
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d*),(\d*) v=(-?\d*),(-?\d*)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, arr)| arr.map(|x| x.parse::<i32>().unwrap()))
        .map(|[pos_x, pos_y, vel_x, vel_y]| Robot {
            pos: (pos_x, pos_y),
            vel: (vel_x, vel_y),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
        assert_eq!(process(input), 12)
    }
}
