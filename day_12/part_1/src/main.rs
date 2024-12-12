use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Default, Debug)]
struct Region {
    area: u32,
    perimeter: u32,
}

impl Region {
    fn calculate_price(&self) -> u32 {
        self.area * self.perimeter
    }
}

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> u32 {
    let map = region_map(
        &input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    );

    let mut regions: HashMap<u32, Region> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let r = regions.entry(map[i][j]).or_default();
            r.area += 1;

            for dir in DIRECTIONS {
                let pos = (dir.0 + i as isize, dir.1 + j as isize);

                if !valid_pos((map.len(), map[0].len()), &pos)
                    || map[pos.0 as usize][pos.1 as usize] != map[i][j]
                {
                    r.perimeter += 1;
                }
            }
        }
    }

    regions.values().map(|r| r.calculate_price()).sum()
}

fn valid_pos(map_size: (usize, usize), pos: &(isize, isize)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < map_size.0 as isize && pos.1 < map_size.1 as isize
}

fn region_map(map: &[Vec<char>]) -> Vec<Vec<u32>> {
    let mut region_map = vec![vec![0; map[0].len()]; map.len()];

    let mut region_counter = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if region_map[i][j] == 0 {
                region_counter += 1;
                flood(
                    map,
                    &mut region_map,
                    (i as isize, j as isize),
                    region_counter,
                    map[i][j],
                );
            }
        }
    }

    region_map
}

fn flood(
    map: &[Vec<char>],
    region_map: &mut Vec<Vec<u32>>,
    pos: (isize, isize),
    region_id: u32,
    current_plant: char,
) {
    region_map[pos.0 as usize][pos.1 as usize] = region_id;

    around_pos(map, region_map, pos, current_plant)
        .iter()
        .for_each(|x| flood(map, region_map, *x, region_id, current_plant));
}

fn around_pos(
    map: &[Vec<char>],
    region_map: &[Vec<u32>],
    pos: (isize, isize),
    current_plant: char,
) -> Vec<(isize, isize)> {
    DIRECTIONS
        .iter()
        .map(|d| (d.0 + pos.0, d.1 + pos.1))
        .filter(|pos| {
            valid_pos((region_map.len(), region_map[0].len()), pos)
                && map[pos.0 as usize][pos.1 as usize] == current_plant
                && region_map[pos.0 as usize][pos.1 as usize] == 0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;
        assert_eq!(process(input), 140)
    }

    #[test]
    fn medium_example() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
        assert_eq!(process(input), 772)
    }

    #[test]
    fn big_example() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        assert_eq!(process(input), 1930)
    }
}
