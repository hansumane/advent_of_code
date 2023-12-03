use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open(args().collect::<Vec<_>>().get(1).unwrap()).unwrap();
    let br = BufReader::new(file);

    let numerical = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];

    let translate: HashMap<&str, u64> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut res1 = 0u64;
    let mut res2 = 0u64;

    for line in br.lines() {
        match line {
            Ok(line) => {
                /* first part */

                /*
                 * just filter the line by only numeric chars
                 * then parse each numeric character into a u64
                 * and get the first one and the last one
                 */

                let numbers = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .map(|c| c.to_string().parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                let a = numbers.first().unwrap();
                let b = numbers.last().unwrap();
                res1 += a * 10 + b;

                /* second part */

                /*
                 * ugh...
                 *
                 * for each value in numerical ["0", "1", ..., "one", "two"]
                 * find all its indices in a line and filter to only non-empty values [index, string]
                 * then add each of these values into the res vector.
                 *
                 * after that, sort res by index and collect only strings
                 * then get first and last string and parse them into a number using translate hashmap
                 *
                 * P.S. I've already looked through other people's solutions and this one
                 * seems like a very bad one. I really hope I will get a bit better
                 * by the end of AoC2023 in both programming and Rust. Rust seems
                 * very unfamiliar and different from other programming languages
                 * I used to learn (python, C, C++ and Java)
                 */

                let mut res: Vec<(usize, &str)> = Vec::new();
                for pair in numerical
                    .iter()
                    .map(|n| line.match_indices(n).collect::<Vec<_>>())
                    .filter(|v| !v.is_empty())
                {
                    for v in pair.iter() {
                        res.push(*v);
                    }
                }

                res.sort_by(|(s, ..), (n, ..)| s.partial_cmp(n).unwrap());
                let res = res.iter().map(|(.., v)| *v).collect::<Vec<&str>>();

                let c = res.first().unwrap().chars().collect::<Vec<_>>().get(0).unwrap().to_owned();
                let c = match c {
                    '0'..='9' => c.to_string().parse::<u64>().unwrap(),
                    _ => *translate.get(res.first().unwrap()).unwrap(),
                };

                let d = res.last().unwrap().chars().collect::<Vec<_>>().get(0).unwrap().to_owned();
                let d = match d {
                    '0'..='9' => d.to_string().parse::<u64>().unwrap(),
                    _ => *translate.get(res.last().unwrap()).unwrap(),
                };

                res2 += c * 10 + d;
            }
            Err(_) => break,
        }
    }

    println!("1) {res1}");
    println!("2) {res2}");
}
