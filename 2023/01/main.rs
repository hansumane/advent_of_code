use std::{
    fs::File,
    env::args,
    io::{BufRead, BufReader},
    collections::HashMap
};

fn main() {
    let file = File::open(args().collect::<Vec<_>>().get(1).unwrap()).unwrap();
    let br = BufReader::new(file);

    let numerical = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                     "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let translate: HashMap<&str, u64> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ].iter().cloned().collect();

    let mut res1 = 0u64;
    let mut res2 = 0u64;

    for line in br.lines() {
        match line {
            Ok(line) => {
                /* first part */

                let numbers = line.chars()
                    .filter(|c| c.is_numeric())
                    .map(|c| c.to_string().parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                let a = numbers.first().unwrap();
                let b = numbers.last().unwrap();
                res1 += a * 10 + b;

                /* second part */

                let mut res: Vec<(usize, &str)> = Vec::new();
                for pair in numerical.iter().map(|n| line.match_indices(n).collect::<Vec<_>>()).filter(|v| !v.is_empty()) {
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
            },
            Err(_) => break,
        }
    }

    println!("1) {res1}");
    println!("2) {res2}");
}
