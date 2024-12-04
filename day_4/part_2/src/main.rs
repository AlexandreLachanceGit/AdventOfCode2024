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
            if word_search[i][j] == 'A' && search(&word_search, (i, j)) {
                count += 1;
            }
        }
    }

    count
}

fn search(word_search: &[Vec<char>], pos: (usize, usize)) -> bool {
    if pos.0 == 0
        || pos.1 == 0
        || pos.0 == word_search.len() - 1
        || pos.1 == word_search[0].len() - 1
    {
        return false;
    }

    let mut letters = ['M', 'M', 'S', 'S'];
    for _ in 0..4 {
        if word_search[pos.0 - 1][pos.1 - 1] == letters[0]
            && word_search[pos.0 - 1][pos.1 + 1] == letters[1]
            && word_search[pos.0 + 1][pos.1 + 1] == letters[2]
            && word_search[pos.0 + 1][pos.1 - 1] == letters[3]
        {
            return true;
        }

        letters.rotate_right(1);
    }

    false
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
        assert_eq!(process(input), 9)
    }
}
