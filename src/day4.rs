pub(crate) fn day4(file_path: &str) {
    let lines = std::fs::read_to_string(file_path)
        .expect("Should read file");
    let lines = lines.lines().collect();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &Vec<&str>) -> i32 {
    lines.iter()
        .map(|line| {
            let values: Vec<i16> = line.replace("-", ",")
                .split(",")
                .map(|num| num.parse().expect("Should read number"))
                .collect();
            return if (values[0] <= values[2] && values[1] >= values[3]) ||
                (values[0] >= values[2] && values[1] <= values[3]) {
                1
            } else {
                0
            }
        }).sum()
}

fn part2(lines: &Vec<&str>) -> i32 {
    lines.iter()
        .map(|line| {
            let values: Vec<i16> = line.replace("-", ",")
                .split(",")
                .map(|num| num.parse().expect("Should read number"))
                .collect();
            return if (values[0] <= values[2] && values[2] <= values[1]) ||
                (values[2] <= values[0] && values[0] <= values[3]) {
                1
            } else {
                0
            }
        }).sum()
}
