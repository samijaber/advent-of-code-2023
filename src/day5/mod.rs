use std::fs;

struct Map {
    dest: i64,
    src: i64,
    length: i64,
}

type Maps = Vec<Vec<Map>>;

struct State {
    seeds: Vec<i64>,
    maps: Maps,
}

fn collect_state(data: String) -> State {
    let mut lines = data.lines();

    let mut maps: Maps = vec![];

    let seeds = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| {
            x.parse::<i64>().unwrap_or_else(|k| {
                println!("fail to parse: {}", k);
                0
            })
        })
        .collect::<Vec<_>>();

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

pub fn part1() -> i64 {
    let practice_data = fs::read_to_string("src/day5/data.txt").expect("Unable to read file");
    // println!("{}", practice_data);

    let state = collect_state(practice_data);

    // for maps in &state.maps {
    //     println!("another map");
    //     for map in maps {
    //         println!(
    //             "Src: {} | Dest: {} | Len: {}",
    //             map.src, map.dest, map.length
    //         );
    //     }
    // }

    let k = state.seeds.iter().map(|seed| {
        return state.maps.as_slice().iter().fold(*seed, |acc, maps| {
            let found_map = maps
                .iter()
                .find(|m| acc >= m.src && acc <= (m.src + m.length));

            match found_map {
                Some(map) => {
                    let new_seed = (map.dest + (acc - &map.src));
                    // println!("updating seed from {} to {}", acc, new_seed);
                    return new_seed;
                }
                None => return acc,
            }
        });
    });

    // for l in k {
    //     println!("final: {}", l)
    // }

    return k.min().unwrap();
}

pub fn main() {
    let total_2 = part1();

    println!("minimum: {}", total_2);
}
