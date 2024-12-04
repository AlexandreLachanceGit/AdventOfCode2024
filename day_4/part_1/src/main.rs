const INPUT: &str = include_str!("../input.txt");

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
                count += search(&word_search, (i, j));
            }
        }
    }

    count
}

fn search(word_search: &[Vec<char>], pos: (usize, usize)) -> usize {
    let letters = ['M', 'A', 'S'];

    let mut found = 0;

    // horizontal
    let mut works = true;
    for (i, l) in letters.iter().enumerate() {
        if pos.1 + i + 1 >= word_search[pos.0].len() || &word_search[pos.0][pos.1 + i + 1] != l {
            works = false;
            break;
        }
    }
    if works {
        found += 1;
    }

    let mut works = true;
    for (i, l) in letters.iter().enumerate() {
        if pos.1 as isize - i as isize - 1 < 0 || &word_search[pos.0][pos.1 - i - 1] != l {
            works = false;
            break;
        }
    }
    if works {
        found += 1;
    }

    // vertical
    let mut works = true;
    for (i, l) in letters.iter().enumerate() {
        if pos.0 + i + 1 >= word_search.len() || &word_search[pos.0 + i + 1][pos.1] != l {
            works = false;
            break;
        }
    }
    if works {
        found += 1;
    }

    let mut works = true;
    for (i, l) in letters.iter().enumerate() {
        if pos.0 as isize - i as isize - 1 < 0 || &word_search[pos.0 - i - 1][pos.1] != l {
            works = false;
            break;
        }
    }
    if works {
        found += 1;
    }

    // diagonal right-up
    let mut works = true;
    for (i, l) in letters.iter().enumerate() {
        if pos.0 + i + 1 >= word_search.len()
            || pos.1 + i + 1 >= word_search[pos.0].len()
            || &word_search[pos.0 + i + 1][pos.1 + i + 1] != l
        {
            works = false;
            break;
        }
    }
    if works {
        found += 1;
    }

    // diagonal right-down
    let mut works = true;
    for (i, l) in letters.iter().enumerate() {
        if pos.0 as isize - i as isize - 1 < 0
            || pos.1 + i + 1 >= word_search[pos.0].len()
            || &word_search[pos.0 - i - 1][pos.1 + i + 1] != l
        {
            works = false;
            break;
        }
    }
    if works {
        found += 1;
    }

    // diagonal left-up
    let mut works = true;
    for (i, l) in letters.iter().enumerate() {
        if pos.0 + i + 1 >= word_search.len()
            || pos.1 as isize - i as isize - 1 < 0
            || &word_search[pos.0 + i + 1][pos.1 - i - 1] != l
        {
            works = false;
            break;
        }
    }
    if works {
        found += 1;
    }

    // diagonal left-down
    let mut works = true;
    for (i, l) in letters.iter().enumerate() {
        if pos.0 as isize - i as isize - 1 < 0
            || pos.1 as isize - i as isize - 1 < 0
            || &word_search[pos.0 - i - 1][pos.1 - i - 1] != l
        {
            works = false;
            break;
        }
    }
    if works {
        found += 1;
    }

    found
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
