use itertools::Itertools;
use std::fs;

struct Map {
    dest: i64,
    src: i64,
    length: i64,
}

type Maps = Vec<Vec<Map>>;

type Seeds = Vec<i64>;
struct State {
    seeds: Seeds,
    maps: Maps,
}

fn collect_seeds_part_1(seed_line: &str) -> Seeds {
    seed_line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn collect_seeds_part_2(seed_line: &str) -> Seeds {
    let binding = seed_line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect_vec();

    println!("binding: {:?}", binding);
    let mut vec: Seeds = vec![];
    binding.chunks(2).for_each(|w| {
        println!("w: {:?}", w);
        vec.append(&mut (w[0]..w[0] + w[1]).collect_vec())
    });

    println!("Seeds found: {:?}", vec.clone());

    // vec.into_iter().unique().collect()

    vec
}

fn collect_state(data: String) -> State {
    let mut lines = data.lines();

    let mut maps: Maps = vec![];

    println!("collecting seeds");
    let seeds = collect_seeds_part_2(lines.next().unwrap());

    // println!("total seeds: {}", seeds.len());
    // seeds = seeds.iter().unique().collect_vec();
    println!("unique seeds: {}", seeds.len());
    for line in lines {
        if line.contains("map") {
            maps.push(vec![]);
        } else if line.matches(char::is_numeric).count() > 0 {
            let mut line_data = line.split_whitespace().map(|x| x.parse::<i64>().unwrap());

            let dest = line_data.next().unwrap();
            let src = line_data.next().unwrap();
            let length = line_data.next().unwrap();
            let map = Map { dest, length, src };

            maps.last_mut().unwrap().push(map);
        }
    }

    return State { seeds, maps };
}

pub fn main() {
    let practice_data = fs::read_to_string("src/day5/example.txt").expect("Unable to read file");

    let state = collect_state(practice_data);

    let k = state.seeds.iter().map(|seed| {
        state.maps.as_slice().iter().fold(*seed, |acc, maps| {
            let found_map = maps
                .iter()
                .find(|m| acc >= m.src && acc <= (m.src + m.length));

            match found_map {
                Some(map) => map.dest + (acc - &map.src),
                None => acc,
            }
        })
    });

    let total_2 = k.min().unwrap();

    println!("minimum: {}", total_2);
}
