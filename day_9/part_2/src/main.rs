// horrible solution

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, PartialEq, Debug)]
enum DiskUnit {
    File { id: usize, size: usize },
    Empty { size: usize },
}

impl DiskUnit {
    fn get_id(&self) -> Option<usize> {
        match self {
            DiskUnit::File { id, size: _ } => Some(*id),
            DiskUnit::Empty { size: _ } => None,
        }
    }

    fn get_size(&self) -> usize {
        match self {
            DiskUnit::File { id: _, size } => *size,
            DiskUnit::Empty { size } => *size,
        }
    }

    fn empty(&mut self) {
        match self {
            DiskUnit::File { id: _, size } => *self = DiskUnit::Empty { size: *size },
            DiskUnit::Empty { size: _ } => {}
        }
    }
}

fn main() {
    println!("Answer: {}", process(INPUT));
}

fn process(input: &str) -> usize {
    let mut blocks = load_blocks(input);
    compact(&mut blocks);

    checksum(&blocks)
}

fn load_blocks(disk_map: &str) -> Vec<DiskUnit> {
    let disk_map = disk_map
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize);
    let mut blocks: Vec<DiskUnit> = vec![];

    let mut is_file = true;
    let mut id: usize = 0;
    for size in disk_map {
        if is_file {
            blocks.push(DiskUnit::File { id, size });
            id += 1;
        } else {
            blocks.push(DiskUnit::Empty { size });
        }

        is_file = !is_file;
    }

    blocks
}

fn compact(units: &mut Vec<DiskUnit>) {
    let mut last: Option<usize> = None;

    while let Some(next_index) = next_to_move(last, units) {
        let next = units[next_index];
        last = Some(next.get_id().unwrap());

        let mut to_insert: Option<(usize, DiskUnit)> = None;
        let mut to_empty: Option<usize> = None;

        for (i, u) in units.iter_mut().enumerate() {
            match u {
                DiskUnit::File { id, size: _ } => {
                    if *id == next.get_id().unwrap() {
                        break;
                    }
                }
                DiskUnit::Empty { size } if *size > next.get_size() => {
                    to_insert = Some((
                        i + 1,
                        DiskUnit::Empty {
                            size: *size - next.get_size(),
                        },
                    ));
                    to_empty = Some(next_index);
                    *u = next;
                    break;
                }
                DiskUnit::Empty { size } if *size == next.get_size() => {
                    *u = next;
                    to_empty = Some(next_index);
                    break;
                }
                DiskUnit::Empty { size: _ } => {}
            }
        }

        if let Some(i) = to_empty {
            let u = units.get_mut(i).unwrap();
            u.empty();
        }
        if let Some((i, u)) = to_insert {
            units.insert(i, u);
        }
        *units = merge_empties(units);
    }
}

fn merge_empties(units: &Vec<DiskUnit>) -> Vec<DiskUnit> {
    let mut sum = 0;
    let mut new = vec![];
    for u in units {
        match u {
            DiskUnit::File { id: _, size: _ } => {
                if sum > 0 {
                    new.push(DiskUnit::Empty { size: sum });
                    sum = 0;
                }
                new.push(*u);
            }
            DiskUnit::Empty { size } => sum += size,
        }
    }
    new
}

fn next_to_move(last_id: Option<usize>, units: &[DiskUnit]) -> Option<usize> {
    if last_id.is_some() && last_id.unwrap() == 0 {
        return None;
    }

    if last_id.is_none() {
        return Some(units.len() - 1);
    }

    units.iter().enumerate().find_map(|(i, u)| match u {
        DiskUnit::File { id, size: _ } => {
            if *id == last_id.unwrap() - 1 {
                Some(i)
            } else {
                None
            }
        }
        DiskUnit::Empty { size: _ } => None,
    })
}

fn checksum(blocks: &[DiskUnit]) -> usize {
    let mut total = 0;
    let mut pos = 0;

    for b in blocks {
        match b {
            DiskUnit::File { id, size } => {
                for i in pos..pos + size {
                    total += i * id;
                }
                pos += size;
            }
            DiskUnit::Empty { size } => pos += size,
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"2333133121414131402"#;
        assert_eq!(process(input), 2858)
    }
}
