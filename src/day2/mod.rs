use crate::day2::data::MAXES;

pub mod data;

pub fn main() {
    let data_set = data::REAL_DATA;
    let mut total = 0;
    for line in data_set.split('\n') {
        let game_line = line.split(':').collect::<Vec<_>>();
        let game_index = game_line[0].split_whitespace().collect::<Vec<_>>()[1];
        let game = game_line[1];

        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        game.split(';').for_each(|round| {
            round.trim().split(',').for_each(|x| {
                let xk = x.trim().split(' ').collect::<Vec<_>>();
                let val = xk[0].parse::<u32>().unwrap();
                if xk[1] == "red" {
                    max_red = max_red.max(val)
                } else if xk[1] == "green" {
                    max_green = max_green.max(val)
                } else {
                    max_blue = max_blue.max(val)
                };
            })
        });

        let power = max_red * max_green * max_blue;

        println!("{}: {}", game_index, power);

        total += power;
    }
    println!("total: {}", total);
}
