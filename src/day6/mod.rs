use core::panic;
use itertools::Itertools;
use std::fs;

pub fn main() {
    let real_data = fs::read_to_string("src/day6/data.txt").expect("Unable to read file");
    let example_data = fs::read_to_string("src/day6/example.txt").expect("Unable to read file");

    let data = real_data;

    // let mut lines = data.lines().map(|line| {
    //     line.split_once(':')
    //         .unwrap()
    //         .1
    //         .trim()
    //         .split_ascii_whitespace()
    //         .map(|n| n.trim().parse::<i64>().unwrap())
    // });

    // let times = lines.next().unwrap();
    // let records = lines.next().unwrap();

    // let races = times.zip(records);

    // let res = races
    //     .map(|(t, r)| {
    //         (1..t)
    //             .flat_map(move |x| if ((t - x) * x) > r { Some(1) } else { None })
    //             .count()
    //     })
    //     .reduce(|k, l| k * l)
    //     .unwrap();
    // println!("{}", res);

    let mut lines = data.lines().map(|line| {
        line.split_once(':')
            .unwrap()
            .1
            .trim()
            .split_ascii_whitespace()
            .join("")
            .trim()
            .parse::<i64>()
            .unwrap()
    });

    let time = lines.next().unwrap();
    let record = lines.next().unwrap();

    let mut found = false;
    let mut index = time / 2;
    let mut step = index;

    while !found {
        let is_above = ((time - index) * index) >= record;
        let prev = index - 1;
        let below_is_below = ((time - prev) * prev) <= record;

        println!("checking: {}", index);

        println!("{} || {} || {}", ((time - index) * index), record, is_above);
        println!(
            "{} || {} || {}",
            ((time - prev) * prev),
            record,
            below_is_below
        );
        found = is_above & below_is_below;

        if !found {
            step = step / 2;
            println!("index: {} || step: {}", index, step);
            if is_above && !below_is_below {
                index = index - step
            } else {
                index = index + step
            }
            if index == 0 {
                panic!("something went wrong.");
            }
        }
    }

    let res = time - index - index + 1;
    println!("res: {}", res);
}
