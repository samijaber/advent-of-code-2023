pub mod data;

fn get_card_matches(card: &str) -> usize {
    let mut cards = card
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split('|')
        .map(|x: &str| {
            x.trim()
                .split(' ')
                .filter_map(|x: &str| x.trim().parse::<i32>().ok())
        });

    let mut winner = cards.next().unwrap();

    return cards
        .next()
        .unwrap()
        .filter(|x| winner.find(|y| x == y).is_some())
        .count();
}

pub fn part1() {
    let data_set = data::PRACTICE_STRING_1;
    let lines = data_set.lines();

    let total: i32 = lines
        .map(|line| {
            vec![0 as usize; get_card_matches(line)]
                .iter()
                .fold(0, |acc, _| match acc {
                    0 => 1,
                    _ => acc * 2,
                })
        })
        .sum();

    println!("total: {}", total)
}

pub fn part2() {
    let data_set = data::REAL_DATA;
    let cards = data_set.lines();

    let matches = cards
        .clone()
        .map(|x| get_card_matches(x))
        .collect::<Vec<_>>();

    fn recursinator(acc: usize, i: usize, data: Vec<usize>, matches: Vec<usize>) -> usize {
        return match data.as_slice() {
            [] => acc,
            [head, tail @ ..] => {
                let matches_for_card = matches[i];
                let card_matches = head + 1;

                let upgraded_tail = tail
                    .iter()
                    .enumerate()
                    .map(|(i, x)| match i {
                        n if n < matches_for_card => x + card_matches,
                        _ => *x,
                    })
                    .collect();
                return recursinator(acc + card_matches, i + 1, upgraded_tail, matches);
            }
        };
    }

    let final_sum = recursinator(0, 0, vec![0 as usize; cards.count()], matches.clone());

    println!("final sum: {}", final_sum);
}

pub fn main() {
    part2();
}
