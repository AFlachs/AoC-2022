use std::collections::vec_deque::VecDeque;

pub(crate) fn day5(file_path: &str) {
    let file_text = std::fs::read_to_string(file_path)
        .expect("Should read file");

    let crates = file_text.lines().take_while(|line| !line.is_empty());
    let n_stacks: usize = (crates.clone().last().unwrap().len() + 1) / 4;

    let mut stacks = vec![VecDeque::<char>::new(); n_stacks];
    // Initiate crates
    crates.for_each(|line| {
        line.char_indices()
            .filter(|(_, symbol)| symbol.is_alphabetic())
            .for_each(|(idx, symbol)| stacks[(idx-1) / 4].push_front(symbol))
    });

    // Move crates
    file_text.lines().skip_while(|line| !line.is_empty())
        .map(|line| {
            let mut nbs = line.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            nbs[1] = nbs[1] - 1;
            nbs[2] = nbs[2] -1;
            return nbs;
        })
        .for_each(|v| move_crate(&mut stacks, v[0], v[1], v[2]));

    let res = stacks.iter()
        .map(|stack| stack.iter().last().unwrap())
        .collect::<String>();

    println!("Part 1: {}", res);
    println!("Part 2: {}", res);
}

fn move_crate(stacks: &mut Vec<VecDeque<char>>, number: usize, from: usize, to: usize) {
    for _ in 1..number {
        let val = stacks[from].pop_back().unwrap();
        stacks[to].push_back(val);
    }
}
