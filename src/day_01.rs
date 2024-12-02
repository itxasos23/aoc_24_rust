use std::fs;
use std::collections::HashMap;

const FILE_PATH: &str = "inputs/day_01.txt";

pub fn solve() {
    println!("Day 01 -- d1: {}, d2: {}", solve_1(), solve_2());
}

fn solve_1() -> isize {
    let input = fs::read_to_string(FILE_PATH).expect("Can't read input file.");

    let mut list_1: Vec<isize> = vec![];
    let mut list_2: Vec<isize> = vec![];

    for row in input.lines() {
        if row.len() == 0 {continue;}
        let mut nums = row.split("   ");
        list_1.extend(&nums.next().unwrap().parse::<isize>());
        list_2.extend(&nums.next().unwrap().parse::<isize>());
    }

    list_1.sort();
    list_2.sort();

    let mut total_dif = 0;

    for (idx, element) in list_1.into_iter().enumerate() {
        let difference = element - list_2[idx];
        total_dif += difference.abs();
    }
    total_dif
}

fn _parse_input(input: String) -> (Vec<isize>, Vec<isize>) {
    let mut list_1: Vec<isize> = vec![];
    let mut list_2: Vec<isize> = vec![];

    for row in input.lines() {
        if row.len() == 0 {continue;}
        let mut nums = row.split("   ");
        list_1.extend(&nums.next().unwrap().parse::<isize>());
        list_2.extend(&nums.next().unwrap().parse::<isize>());
    }
    (list_1, list_2)
}

fn _build_occurrence_hashmap(_list: &Vec<isize>) -> HashMap<isize, isize> {
    let mut _map: HashMap<isize, isize> = HashMap::new();

    for item in _list {
        if _map.contains_key(&item) {
            _map.insert(*item, _map.get(&item).unwrap() + 1);
        } else {
            _map.insert(*item, 1);
        }
    }
    _map
    
}

fn solve_2() -> isize {
    let input = fs::read_to_string(FILE_PATH).expect("Can't read input file.");

    let (list_1, list_2) = _parse_input(input);

    let map_1 = _build_occurrence_hashmap(&list_1);
    let map_2 = _build_occurrence_hashmap(&list_2);

    let mut total_dif = 0;
    for (element, occurrence_1) in &map_1 {
        let oc_2 = map_2.get(element).unwrap_or(&0);
        let difference = *element * occurrence_1 * oc_2;
        total_dif += difference.abs();
    }
    total_dif

}
