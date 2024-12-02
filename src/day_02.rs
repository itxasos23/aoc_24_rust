use std::fs;

const FILE_PATH: &str = "inputs/day_02.txt";

pub fn solve() {
    println!("Day 02 -- d1: {}, d2: {}", solve_1(), solve_2());
}

const EXAMPLE_INPUT_STR: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn _parse_input(input: &str) -> Vec<Vec<isize>> {
    let mut levels: Vec<Vec<isize>> = Vec::new();

    for row in input.lines() {
        if row.len() == 0 { continue; }
        let level: Vec<isize> = row.split(" ").map(|r| {r.parse::<isize>().unwrap()}).collect();
        levels.push(level);
    }
    levels
}

fn _is_level_safe(input_level: &Vec<isize>) -> bool {
    let mut level = input_level.clone();
    if input_level.first() > input_level.last() {level.reverse();}

    let mut pivot = level[0];
    for item in level[1..].iter() {
        if 1 > (item - pivot) || (item-pivot) > 3 {
            return false
        }
        pivot = *item;
    }
    return true
}

fn _is_any_level_safe(input_level: Vec<isize>) -> bool {

    for idx in 0..input_level.len() {
        let mut this_level: Vec<isize> = vec![];
        this_level.extend(&input_level[0..idx]);
        this_level.extend(&input_level[idx+1..]);

        if _is_level_safe(&this_level) { return true }
    }
    return false
}

fn solve_1() -> isize {
    let input = fs::read_to_string(FILE_PATH).expect("Can't read input file.");
    let levels = _parse_input(&input);
    let mut safe_level_count = 0;

    for level in levels {
        if _is_level_safe(&level) {
            safe_level_count += 1;
        }
    }
    safe_level_count
}


fn solve_2() -> isize {
    let input = fs::read_to_string(FILE_PATH).expect("Can't read input file.");
    let levels = _parse_input(&input);
    let mut safe_level_count = 0;

    for level in levels {
        if _is_level_safe(&level) || _is_any_level_safe(level) {
            safe_level_count += 1;
            continue;
        }
    }
    safe_level_count
}
