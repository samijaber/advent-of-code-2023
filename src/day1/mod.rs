pub mod data;

struct Index {
    first_index: Option<usize>,
    last_index: Option<usize>,
    value: &'static str,
}

struct IndexInit {
    name: &'static str,
    value: &'static str,
}

const IDX_NAMES: [IndexInit; 9] = [
    IndexInit {
        name: "one",
        value: "1",
    },
    IndexInit {
        name: "two",
        value: "2",
    },
    IndexInit {
        name: "three",
        value: "3",
    },
    IndexInit {
        name: "four",
        value: "4",
    },
    IndexInit {
        name: "five",
        value: "5",
    },
    IndexInit {
        name: "six",
        value: "6",
    },
    IndexInit {
        name: "seven",
        value: "7",
    },
    IndexInit {
        name: "eight",
        value: "8",
    },
    IndexInit {
        name: "nine",
        value: "9",
    },
];

pub fn main() {
    let mut total = 0;
    for line in data::REAL_DATA.split_whitespace() {
        // println!("{}", line);

        let idxs = IDX_NAMES.map(|x| Index {
            value: x.value,
            first_index: line.find(x.name),
            last_index: line.rfind(x.name),
        });

        let mut first_arr = idxs
            .iter()
            .filter(|x| x.first_index.is_some())
            .collect::<Vec<_>>();
        first_arr.sort_by_key(|x| x.first_index.unwrap());
        let first_match = first_arr.first();

        let mut last_arr = idxs
            .iter()
            .filter(|x| x.last_index.is_some())
            .collect::<Vec<_>>();
        last_arr.sort_by_key(|x| x.last_index.unwrap());
        let last_match = last_arr.last();

        let first_digit_index = line.find(char::is_numeric);
        let last_digit_index = line.rfind(char::is_numeric);

        let final_first = if first_digit_index
            .is_some_and(|x| first_match.is_none() || x < first_match.unwrap().first_index.unwrap())
        {
            line.get(first_digit_index.unwrap()..first_digit_index.unwrap() + 1)
                .unwrap()
        } else {
            first_match.unwrap().value
        };

        let final_last = if last_digit_index
            .is_some_and(|f| last_match.is_none() || f > last_match.unwrap().last_index.unwrap())
        {
            line.get(last_digit_index.unwrap()..last_digit_index.unwrap() + 1)
                .unwrap()
        } else {
            last_match.unwrap().value
        };

        let sum = [final_first, final_last].join("").parse::<i32>().unwrap();

        total += sum;
        // println!("sum is: {}", sum);
    }
    println!("total: {}", total);
}
