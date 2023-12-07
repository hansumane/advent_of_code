use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let br = BufReader::new(File::open(args().collect::<Vec<_>>().get(1).unwrap()).unwrap());
    let lines = br
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let times = lines
        .first()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .split_first()
        .unwrap()
        .1
        .iter()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let distances = lines
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .split_first()
        .unwrap()
        .1
        .iter()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let all_time = lines
        .first()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .split_first()
        .unwrap()
        .1
        .join("")
        .parse::<i64>()
        .unwrap();

    let all_distance = lines
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .split_first()
        .unwrap()
        .1
        .join("")
        .parse::<i64>()
        .unwrap();

    let pairs = times.into_iter().zip(distances).collect::<Vec<_>>();

    let mut res1 = 1i64;

    for (t, d) in pairs {
        let mut lc = 0i64;
        for st in 1..t {
            if st * (t - st) > d {
                lc += 1;
            }
        }
        res1 *= lc;
    }

    let mut res2 = 0i64;

    for st in 1..all_time {
        if st * (all_time - st) > all_distance {
            res2 += 1;
        }
    }

    println!("1) {res1}");
    println!("2) {res2}");
}
