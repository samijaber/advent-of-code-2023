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

    let winner = cards.nth(0).unwrap();
    let mine = cards.nth(0).unwrap();

    let matches = mine
        .filter(|x: &i32| {
            return winner.clone().find(|y| x == y).is_some();
        })
        .count();

    return matches;
}

pub fn part1() {
    let data_set = data::PRACTICE_STRING_1;
    let lines = data_set.lines().enumerate();

    let mut total = 0;
    for (_i, line) in lines {
        let matches = get_card_matches(line);

        let _power = [0; 1]
            .iter()
            .cycle()
            .take(matches)
            .fold(0, |acc, _| match acc {
                0 => 1,
                _ => acc * 2,
            });

        total += _power;
    }

    println!("total: {}", total)
}

pub fn part2() {
    let data_set = data::REAL_DATA;
    let cards = data_set.lines().enumerate();

    let matches = cards
        .clone()
        .map(|(_i, x)| get_card_matches(x))
        .collect::<Vec<_>>();

    fn collector2(acc: usize, i: usize, data: Vec<usize>, matches: Vec<usize>) -> usize {
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
                return collector2(acc + card_matches, i + 1, upgraded_tail, matches);
            }
        };
    }

    let final_sum = collector2(0, 0, cards.map(|_| 0).collect::<Vec<_>>(), matches.clone());

    println!("final sum: {}", final_sum);
}

pub fn main() {
    part2();
}
