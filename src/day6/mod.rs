use std::fs;

pub fn main() {
    let real_data = fs::read_to_string("src/day6/data.txt").expect("Unable to read file");
    let example_data = fs::read_to_string("src/day6/example.txt").expect("Unable to read file");

    let data = real_data;

    let mut lines = data.lines().map(|line| {
        line.split_once(':')
            .unwrap()
            .1
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.trim().parse::<i64>().unwrap())
    });

    let times = lines.next().unwrap();
    let records = lines.next().unwrap();

    let races = times.zip(records);

    let res = races
        .map(|(t, r)| {
            (1..t)
                .flat_map(move |x| if ((t - x) * x) > r { Some(1) } else { None })
                .count()
        })
        .reduce(|k, l| k * l)
        .unwrap();
    println!("{}", res);
}
