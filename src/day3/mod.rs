pub mod data;

struct Coord {
    x: usize,
    y: usize,
    c: char,
}

struct Number {
    x_start: usize,
    x_end: usize,
    y: usize,
    value: String,
}

pub fn main() {
    let data_set = data::REAL_DATA;

    // find all symbols
    let lines = data_set.lines().enumerate();

    let symbols = lines.flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| {
            if c.is_numeric() || c == '.' {
                return None;
            } else {
                return Some(Coord { x, y, c });
            }
        })
    });

    // for symbol in symbols {
    //     println!("symbol {}: {}-{}", symbol.c, symbol.x, symbol.y);
    // }

    let mut numbers = Vec::new();

    data_set.lines().enumerate().for_each(|(y, line)| {
        let char_indices = line.char_indices();

        for (x, c) in char_indices {
            if !c.is_numeric() {
                continue;
            }

            // println!("encountering: {}", c);
            if x > 0 && line.chars().nth(x - 1).is_some_and(|f| f.is_numeric()) {
                let last: &mut Number = numbers.last_mut().unwrap();
                // println!("appending '{}' to last number '{}'", c, last.value);
                last.x_end = x;
                last.value.push(c);
                // println!("last number: {:?}", last.value)
            } else {
                // println!("creating new number {}", c);
                numbers.push(Number {
                    x_start: x,
                    x_end: x,
                    y,
                    value: c.to_string(),
                });
            }
        }
    });

    // for number in &numbers {
    //     println!(
    //         "number: {} at {}-{}:{}",
    //         number.value, number.x_start, number.x_end, number.y
    //     );
    // }

    let used_numbers = numbers.iter().filter(|number| {
        return symbols
            .clone()
            .any(|symbol| num_touches_symbol(number, &symbol));
    });

    let total: i32 = used_numbers.map(|k| k.value.parse::<i32>().unwrap()).sum();
    println!("total: {}", total);

    let gear_sums: i32 = symbols
        .filter_map(|s| {
            if (s.c != '*') {
                return None;
            } else {
                let filter_map =
                    numbers
                        .iter()
                        .filter_map(|number| match num_touches_symbol(number, &s) {
                            true => Some(number.value.parse::<i32>().unwrap()),
                            false => None,
                        });

                if filter_map.clone().count() == 2 {
                    Some(filter_map.reduce(|a, b| a * b).unwrap())
                } else {
                    None
                }
            }
        })
        .sum();

    println!("gear sums: {:?}", gear_sums);
}

fn num_touches_symbol(number: &Number, symbol: &Coord) -> bool {
    let x_start = if number.x_start == 0 {
        0
    } else {
        number.x_start - 1
    };
    let y_start = if number.y == 0 { 0 } else { number.y - 1 };
    let cond = symbol.x >= x_start
        && symbol.x <= number.x_end + 1
        && symbol.y <= number.y + 1
        && symbol.y >= y_start;
    return cond;
}
