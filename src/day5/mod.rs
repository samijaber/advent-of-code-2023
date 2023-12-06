use itertools::Itertools;
use std::{fs, ops::Range, vec};

type Rng = Range<u64>;
struct Map {
    dest_range: Rng,
    src_range: Rng,
}

type Maps = Vec<Vec<Map>>;
type Ranges = Vec<Rng>;

type Seeds = Ranges;
struct State {
    seeds: Seeds,
    maps: Maps,
}

fn collect_seeds_ranges(seed_line: &str) -> Seeds {
    let binding = seed_line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect_vec();

    let mut vec: Seeds = vec![];
    binding.chunks(2).for_each(|w| vec.push(w[0]..w[0] + w[1]));

    // println!("Seeds found: {:?}", vec.clone());

    // vec.into_iter().unique().collect()

    vec
}

fn collect_state(data: String) -> State {
    let mut lines = data.lines();

    let mut maps: Maps = vec![];

    println!("collecting seeds");
    let seeds = collect_seeds_ranges(lines.next().unwrap());

    // println!("total seeds: {}", seeds.len());
    // seeds = seeds.iter().unique().collect_vec();
    println!("seeds: {:?}", seeds);
    for line in lines {
        if line.contains("map") {
            maps.push(vec![]);
        } else if line.matches(char::is_numeric).count() > 0 {
            let mut line_data = line.split_whitespace().map(|x| x.parse::<u64>().unwrap());

            let dest = line_data.next().unwrap();
            let src = line_data.next().unwrap();
            let length = line_data.next().unwrap();
            let map = Map {
                dest_range: (dest..dest + length),
                src_range: (src..src + length),
            };

            // println!("map: {}, {}, {}", map.dest, map.length, map.src);
            maps.last_mut().unwrap().push(map);
        }
    }

    return State { seeds, maps };
}

pub fn main() {
    let practice_data = fs::read_to_string("src/day5/example.txt").expect("Unable to read file");
    let mut state = collect_state(practice_data);

    for map in &mut state.maps {
        // println!("=====================");
        map.sort_by(|a, b| {
            let b_start = b.src_range.start;
            a.src_range.start.cmp(&b_start)
        });
    }

    let output = move_ranges(state.seeds, vec![], state.maps.pop().unwrap(), state.maps);

    println!("output: {}", output);
}

fn move_ranges(
    mut ranges: Ranges,
    mut next_ranges: Ranges,
    curr_maps: Vec<Map>,
    mut tail: Maps,
) -> u64 {
    println!("=========");
    println!("=========");
    println!("RUNNING");
    println!("ranges: {:?}", ranges);
    println!("next_ranges: {:?}", next_ranges);

    match ranges[..] {
        [] => match (next_ranges.as_slice(), tail.pop()) {
            (_, Some(new_maps)) => {
                println!("moving to next ranges");
                move_ranges(next_ranges, vec![], new_maps, tail)
            }
            (_, _) => {
                println!("base case! {:?} {:?}", ranges, next_ranges);
                next_ranges.iter().map(|k| k.start).min().unwrap()
            }
        },

        _ => {
            let range = ranges.pop().unwrap();
            println!("processing range {:?}", range);
            for m in &curr_maps {
                println!("in maps: {:?} --> {:?}", m.src_range, m.dest_range);
            }

            let _found = curr_maps.iter().find(|m| {
                let item = m.src_range.end - 1;
                range.contains(&m.src_range.start) || range.contains(&item)
            });

            match _found {
                Some(Map {
                    dest_range,
                    src_range,
                }) => {
                    println!("found: {:?}", src_range);

                    let overlap = src_range.start.max(range.start)..src_range.end.min(range.end);
                    let pre_range = range.start..overlap.start;
                    let post_range = overlap.end..range.end;

                    let dist = overlap.start - src_range.start;
                    let mapped_overlap =
                        dest_range.start + dist..dest_range.start + dist + overlap.count() as u64;

                    if !mapped_overlap.is_empty() {
                        println!("mapped_overlap: {:?}", mapped_overlap);
                        next_ranges.push(mapped_overlap);
                    }
                    if !pre_range.is_empty() {
                        println!("adding pre_range: {:?}", pre_range);
                        ranges.push(pre_range);
                    }

                    if !post_range.is_empty() {
                        println!("adding post_range: {:?}", post_range);
                        ranges.push(post_range);
                    }
                }
                None => {
                    next_ranges.push(range);
                }
            }
            move_ranges(ranges, next_ranges, curr_maps, tail)
        }
    }
}
