pub mod data;

pub fn main() {
    let data_set = data::REAL_DATA;

    let lines = data_set.lines().enumerate();

    let mut total = 0;
    for (_i, line) in lines {
        println!("line: {}", line);
        let trim = &line.split(':').nth(1).unwrap().trim();
        let flt = trim.split('|');

        // println!("flt: {:?}", trim);
        let mut cards = flt.map(|x| {
            x.trim()
                .split(' ')
                .filter_map(|x| x.trim().parse::<i32>().ok())
        });

        let winner = cards.nth(0).unwrap();
        let mine = cards.nth(0).unwrap();

        let matches = mine
            .filter(|x| {
                return winner.clone().find(|y| x == y).is_some();
            })
            .fold(0, |acc, _| match acc {
                0 => 1,
                _ => acc * 2,
            });

        total += matches;
    }

    println!("total: {}", total)
}
