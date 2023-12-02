use crate::day2::data::MAXES;

pub mod data;

pub fn main() {
    let data_set = data::REAL_DATA;
    let mut total = 0;
    for line in data_set.split('\n') {
        let game_line = line.split(':').collect::<Vec<_>>();
        let game_index = game_line[0].split_whitespace().collect::<Vec<_>>()[1];
        let game = game_line[1];

        let is_possible = game.split(';').all(|round| {
            round.trim().split(',').all(|x| {
                let xk = x.trim().split(' ').collect::<Vec<_>>();
                let max = if xk[1] == "red" {
                    MAXES.red
                } else if xk[1] == "green" {
                    MAXES.green
                } else {
                    MAXES.blue
                };
                return max >= xk[0].parse::<i32>().unwrap();
            })
        });

        if is_possible {
            total += game_index.parse::<i32>().unwrap();
        }
        println!("#{} is possible: {}", game_index, is_possible);

        // println!("\tRound: {} is possible: {}", is_possible)
        // for round in game.split(';') {
        // }
    }
    println!("total: {}", total);
}
