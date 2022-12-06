use std::fs;

struct Item {
    value: usize,
}

impl From<&u8> for Item {
    fn from(value: &u8) -> Self {
        let value = if *value > b'a' {
            *value as u8 - b'a' + 1
        } else {
            *value as u8 - b'A' + 27
        };
        Self {value: value as usize}
    }
}

pub(crate) fn day3(file_path: &str) {
    let lines = fs::read_to_string(file_path)
        .expect("Should read file");
    let lines = lines.lines().collect();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(input: &Vec<&str>) -> usize {
    let mut sum = 0usize;

    for values in input{
        let (part1, part2) = values.as_bytes().split_at(values.len() / 2);

        let mut occurrences: [bool; 53] = [false; 53]; // Array with one entry per char
        for x in part1 {
            let item = Item::from(x);
            occurrences[item.value] = true;
        }

        for x in part2 {
            let item = Item::from(x);
            if occurrences[item.value] {
                sum += item.value;
                break;
            }
        }
    }
    sum
}

fn part2(input: &[&str]) -> usize {
    input.chunks(3)
        .map(check_group)
        .sum()
}

fn check_group(chunk: &[&str]) -> usize {
    let mut occurrences: [u8; 53] = [0; 53];
    let mut sum = 0;

    for (row_index, sack) in chunk.iter().enumerate() {
        for element in sack.as_bytes() {
            let item = Item::from(element);

            if row_index < 2 {
                occurrences[item.value] |= 1 << row_index;
                // Start by counting occurrences of every char in the first to sacks
            } else if occurrences[item.value] == 0b011{
                sum += item.value;
                break;
            }
        }
    }
    sum
}