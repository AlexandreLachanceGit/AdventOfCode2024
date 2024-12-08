use std::collections::{HashMap, HashSet};
use std::mem::swap;

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let freq_antennas = find_antennas(&map);

    get_antinode_count((map.len(), map[0].len()), freq_antennas)
}

fn find_antennas(map: &[Vec<char>]) -> HashMap<char, Vec<(isize, isize)>> {
    let mut freq_antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let freq = map[i][j];
            if freq != '.' {
                freq_antennas
                    .entry(freq)
                    .or_default()
                    .push((i as isize, j as isize));
            }
        }
    }
    freq_antennas
}

fn get_antinode_count(
    map_size: (usize, usize),
    freq_antennas: HashMap<char, Vec<(isize, isize)>>,
) -> usize {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    freq_antennas.values().for_each(|v| {
        v.iter()
            .tuple_combinations::<(_, _)>()
            .flat_map(|(a, b)| calculate_antinodes(map_size, *a, *b))
            .for_each(|pos| {
                antinodes.insert(pos);
            })
    });

    antinodes.len()
}

fn calculate_antinodes(
    map_size: (usize, usize),
    mut a: (isize, isize),
    mut b: (isize, isize),
) -> Vec<(isize, isize)> {
    let mut antinodes = vec![];

    let distance = (a.0 - b.0, a.1 - b.1);

    if (a.0 - distance.0, a.1 - distance.1) == b {
        swap(&mut a, &mut b);
    }
    let first = (a.0 - distance.0, a.1 - distance.1);
    if valid_pos(map_size, &first) {
        antinodes.push(first);
    }
    let second = (b.0 + distance.0, b.1 + distance.1);
    if valid_pos(map_size, &second) {
        antinodes.push(second);
    }

    antinodes
}

fn valid_pos(map_size: (usize, usize), pos: &(isize, isize)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < map_size.0 as isize && pos.1 < map_size.1 as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        assert_eq!(process(input), 14)
    }

    #[test]
    fn example_simple() {
        let input = r#"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."#;
        assert_eq!(process(input), 2)
    }

    #[test]
    fn example_three() {
        let input = r#"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."#;
        assert_eq!(process(input), 4)
    }

    #[test]
    fn example_five() {
        let input = r#"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
....0..0.."#;
        assert_eq!(process(input), 5)
    }
}
