use std::collections::HashSet;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

const MAP_SIZE: (i32, i32) = (101, 103);

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn get_final_pos(&self, second: i32) -> (i32, i32) {
        let pos = (
            (second * self.vel.0 + self.pos.0) % MAP_SIZE.0,
            (second * self.vel.1 + self.pos.1) % MAP_SIZE.1,
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
    let mut i = 0;
    loop {
        i += 1;

        let mut set = HashSet::new();
        let mut unique = true;

        parse_robots(input)
            .iter()
            .map(|r| r.get_final_pos(i))
            .for_each(|pos| {
                if set.contains(&pos) {
                    unique = false;
                }
                set.insert(pos);
            });

        if unique {
            let mut map = vec![vec![0; MAP_SIZE.0 as usize]; MAP_SIZE.1 as usize];
            set.iter()
                .for_each(|(x, y)| map[*y as usize][*x as usize] += 1);
            println!();
            for y in 0..MAP_SIZE.1 {
                for x in 0..MAP_SIZE.0 {
                    if map[y as usize][x as usize] > 0 {
                        print!("◻️");
                    } else {
                        print!("◼️");
                    }
                }
                println!();
            }
            return i;
        }
    }
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
