const INPUT: &str = include_str!("../input.txt");

const LETTERS: [char; 3] = ['M', 'A', 'S'];
const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> usize {
    let word_search = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;

    for i in 0..word_search.len() {
        for j in 0..word_search[0].len() {
            if word_search[i][j] == 'X' {
                count += search(&word_search, (i as isize, j as isize));
            }
        }
    }

    count
}

fn search(word_search: &[Vec<char>], pos: (isize, isize)) -> usize {
    DIRECTIONS
        .iter()
        .filter(|dir| search_dir(word_search, pos, **dir))
        .count()
}

fn search_dir(word_search: &[Vec<char>], pos: (isize, isize), dir: (isize, isize)) -> bool {
    for (i, l) in LETTERS.iter().enumerate() {
        let new_pos = (
            pos.0 + (i as isize + 1) * dir.0,
            pos.1 + (i as isize + 1) * dir.1,
        );

        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= word_search.len() as isize
            || new_pos.1 >= word_search[0].len() as isize
            || word_search[new_pos.0 as usize][new_pos.1 as usize] != *l
        {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(process(input), 18)
    }
}
