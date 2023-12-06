use itertools::Itertools;
use std::{collections::HashMap, fs, vec};

struct Map {
    dest: i64,
    src: i64,
    length: i64,
}

type Maps = Vec<Vec<Map>>;
type Maps2 = Vec<HashMap<i64, i64>>;
type Seeds = Vec<i64>;
struct State {
    seeds: Seeds,
    maps: Maps,
    maps2: Maps2,
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

    // println!("binding: {:?}", binding);
    let mut vec: Seeds = vec![];
    binding.chunks(2).for_each(|w| {
        // println!("w: {:?}", w);
        vec.append(&mut (w[0]..w[0] + w[1]).collect_vec())
    });

    // println!("Seeds found: {:?}", vec.clone());

    // vec.into_iter().unique().collect()

    vec
}

fn collect_state(data: String) -> State {
    let mut lines = data.lines();

    let mut maps: Maps = vec![];
    let mut maps2: Maps2 = vec![];

    println!("collecting seeds");
    let seeds = collect_seeds_part_2(lines.next().unwrap());

    // println!("total seeds: {}", seeds.len());
    // seeds = seeds.iter().unique().collect_vec();
    println!("unique seeds: {}", seeds.len());
    for line in lines {
        if line.contains("map") {
            // println!("found map");
            maps2.push(HashMap::new());
            maps.push(vec![]);
        } else if line.matches(char::is_numeric).count() > 0 {
            let mut line_data = line.split_whitespace().map(|x| x.parse::<i64>().unwrap());

            let dest = line_data.next().unwrap();
            let src = line_data.next().unwrap();
            let length = line_data.next().unwrap();
            let map = Map { dest, length, src };

            // println!("map: {}, {}, {}", map.dest, map.length, map.src);
            maps.last_mut().unwrap().push(map);

            // let hash_map = &mut maps2.last_mut().unwrap();

            // println!("inserting {} -> {}. {}", src, dest, length);
            // for i in 0..0 + length {
            //     hash_map.insert(src + i, dest + i);
            // }
        }
    }

    return State { seeds, maps2, maps };
}

pub fn main() {
    let practice_data = fs::read_to_string("src/day5/example.txt").expect("Unable to read file");

    let state = collect_state(practice_data);

    let k = state.seeds.iter().map(|seed| {
        // let mut v = seed;
        // for map in state.maps2.iter() {
        //     v = map.get(v).unwrap_or(v);
        // }
        // v
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
