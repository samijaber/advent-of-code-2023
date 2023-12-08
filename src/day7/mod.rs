use std::{cmp::Ordering, fs};

use itertools::Itertools;

struct Hand<'a> {
    hand: &'a str,
    score: &'a str,
}

const SORT_ORDER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn compare_two_cards(c1: &char, c2: &char) -> Ordering {
    SORT_ORDER
        .iter()
        .find_position(|k| *k == c1)
        .cmp(&SORT_ORDER.iter().find_position(|k| *k == c2))
}

/**
 * Start by comparing the first card in each hand.
 * If these cards are different, the hand with the stronger first card is considered stronger.
 * If the first card in each hand have the same label, however, then move on
 * to considering the second card in each hand.
 * If they differ, the hand with the higher second card wins;
 * otherwise, continue with the third card in each hand, then the fourth, then the fifth.
 */
fn compare_two_hands(Hand { hand: h1, .. }: &Hand, Hand { hand: h2, .. }: &Hand) -> Ordering {
    h1.chars()
        .zip(h2.chars())
        .map(|(c1, c2)| compare_two_cards(&c1, &c2))
        .find(|o| !o.is_eq())
        .unwrap_or(Ordering::Equal)
}

pub fn main() {
    let real_data = fs::read_to_string("src/day7/data.txt").expect("Unable to read file");
    let example_data = fs::read_to_string("src/day7/example.txt").expect("Unable to read file");

    let data = real_data;

    /*
     * - 0 Five of a kind, where all five cards have the same label: AAAAA
     * - 1 Four of a kind, where four cards have the same label and one card has a different label: AA8AA
     * - 2 Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
     * - 3 Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
     * - 4 Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
     * - 5 One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
     * - 6 High card, where all cards' labels are distinct: 23456
     */
    let mut types: Vec<Vec<Hand>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

    data.lines().for_each(|l| {
        let (card, score) = l.split_once(' ').unwrap();

        let mut groups = card
            .chars()
            .sorted()
            .dedup_with_count()
            .sorted_by_key(|l| l.0)
            .rev();

        let jokers = groups
            .clone()
            .find(|(_, v)| *v == 'J')
            .unwrap_or((0, 'J'))
            .0;

        let highest = groups.next().unwrap();
        let second_highest = groups.next().unwrap_or((0, 'Z'));
        let third_highest = groups.next().unwrap_or((0, 'Z'));

        let (n1, n2) = match (highest, second_highest, jokers) {
            ((h, 'J'), (s, _), _) => (h + s, third_highest.0),
            ((h, _), (s, 'J'), _) => (h + s, third_highest.0),
            ((h, _), (s, _), j) => (h + j, s),
        };

        let vec_idx = match (n1, n2) {
            (n, _) if n > 5 => 0,
            (5, _) => 0,

            (4, _) => 1,
            (3, 2) => 2,
            (3, _) => 3,
            (2, 2) => 4,
            (2, _) => 5,
            (_, _) => 6,
        };

        types
            .get_mut(vec_idx)
            .unwrap()
            .push(Hand { hand: card, score })
    });

    types.iter_mut().for_each(|f| {
        f.sort_by(compare_two_hands);
        f.reverse()
    });

    let total = types
        .iter()
        .flatten()
        .rev()
        .enumerate()
        .map(|(i, c)| c.score.parse::<i64>().unwrap() * (i as i64 + 1))
        .sum::<i64>();

    println!("TOTAL: {}", total);
}
