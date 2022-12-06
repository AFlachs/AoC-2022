use std::fs;
use std::collections::BinaryHeap;

pub(crate) fn day1(file_path: &str) -> BinaryHeap<i32> {
    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read file");

    let mut player_calories: i32 = 0;
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    for line in content.lines() {
        let trimmed_line = line.trim();
        if !trimmed_line.is_empty() {
            let line_value: i32 = trimmed_line.parse().expect("Prob parsing");
            println!("{player_calories} triying to add {line_value}");
            player_calories += line_value;
        }
        else {
            heap.push(player_calories);
            player_calories = 0;
        }
    }
    // playerCalories
    heap
}