const INPUT: &str = include_str!("../input.txt");

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> i32 {
    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect::<Vec<Vec<isize>>>();

    let trailheads = find_trailheads(&map);

    trailheads
        .iter()
        .map(|t| calculate_trailhead_score(&map, *t))
        .sum()
}

fn find_trailheads(map: &[Vec<isize>]) -> Vec<(isize, isize)> {
    let mut trailheads = vec![];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                trailheads.push((i as isize, j as isize));
            }
        }
    }
    trailheads
}

fn calculate_trailhead_score(map: &[Vec<isize>], trailhead: (isize, isize)) -> i32 {
    let val = map[trailhead.0 as usize][trailhead.1 as usize];
    if val == 9 {
        return 1;
    }

    around_pos(map, &trailhead)
        .iter()
        .filter(|pos| map[pos.0 as usize][pos.1 as usize] == val + 1)
        .map(|pos| calculate_trailhead_score(map, *pos))
        .sum()
}

fn around_pos(map: &[Vec<isize>], pos: &(isize, isize)) -> Vec<(isize, isize)> {
    DIRECTIONS
        .iter()
        .map(|d| (d.0 + pos.0, d.1 + pos.1))
        .filter(|pos| valid_pos((map.len(), map[0].len()), pos))
        .collect()
}

fn valid_pos(map_size: (usize, usize), pos: &(isize, isize)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < map_size.0 as isize && pos.1 < map_size.1 as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        assert_eq!(process(input), 81)
    }
}
