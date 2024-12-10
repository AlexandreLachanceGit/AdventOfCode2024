const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> usize {
    let blocks = compact(&load_blocks(input));

    checksum(&blocks)
}

fn load_blocks(disk_map: &str) -> Vec<Option<usize>> {
    let disk_map = disk_map.chars().filter_map(|c| c.to_digit(10));
    let mut blocks: Vec<Option<usize>> = vec![];

    let mut is_file = true;
    let mut id: usize = 0;
    for size in disk_map {
        if is_file {
            blocks.append(&mut vec![Some(id); size as usize]);
            id += 1;
        } else {
            blocks.append(&mut vec![None; size as usize]);
        }

        is_file = !is_file;
    }

    blocks
}

fn compact(blocks: &Vec<Option<usize>>) -> Vec<usize> {
    let mut compact = vec![];

    let mut to_move_pos = next_block_to_move(None, blocks);
    for (i, block) in blocks.iter().enumerate() {
        if i > to_move_pos {
            break;
        }

        if let Some(b) = block {
            compact.push(*b);
        } else {
            compact.push(blocks[to_move_pos].unwrap());
            to_move_pos = next_block_to_move(Some(to_move_pos), blocks);
        }
    }

    compact
}

fn next_block_to_move(last_moved: Option<usize>, blocks: &Vec<Option<usize>>) -> usize {
    let mut pos = last_moved.unwrap_or(blocks.len()) - 1;

    while blocks[pos].is_none() {
        pos -= 1;
    }

    pos
}

fn checksum(blocks: &[usize]) -> usize {
    blocks
        .iter()
        .enumerate()
        .fold(0, |acc, (pos, block_id)| acc + pos * *block_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"2333133121414131402"#;
        assert_eq!(process(input), 1928)
    }
}
