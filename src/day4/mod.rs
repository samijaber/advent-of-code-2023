pub mod data;

fn get_card_matches(card: &str) -> usize {
    let (winner, owned) = card.split_once(':').unwrap().1.split_once('|').unwrap();

    let winner = winner.split_whitespace().collect::<Vec<_>>();
    return owned
        .split_whitespace()
        .filter(|x| winner.contains(x))
        .count();
}

pub fn part1() -> i32 {
    return data::PRACTICE_STRING_1
        .lines()
        .map(|line| {
            vec![0 as usize; get_card_matches(line)]
                .iter()
                .fold(0, |acc, _| match acc {
                    0 => 1,
                    _ => acc * 2,
                })
        })
        .sum();
}

pub fn part2() -> usize {
    let matches = data::REAL_DATA
        .lines()
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

    return recursinator(0, 0, vec![0 as usize; matches.len()], matches.clone());
}

pub fn main() {
    let total_2 = part2();
}
