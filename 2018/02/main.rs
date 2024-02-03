use std::{collections::HashMap, env::args, fs::read_to_string};

fn main() {
    let input = read_to_string(args().collect::<Vec<_>>().get(1).unwrap()).unwrap();
    let lines = input
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let mut ctr2 = 0u64;
    let mut ctr3 = 0u64;

    for line in lines.iter() {
        let mut p2 = false;
        let mut p3 = false;
        let mut lhm = HashMap::<char, u64>::new();

        for c in line.chars() {
            if let Some(v) = lhm.get(&c) {
                lhm.insert(c, *v + 1);
            } else {
                lhm.insert(c, 1);
            }
        }

        for (_, v) in lhm.iter() {
            if *v == 2 && !p2 {
                ctr2 += 1;
                p2 = true;
            } else if *v == 3 && !p3 {
                ctr3 += 1;
                p3 = true;
            }
        }
    }

    println!("res1 = {}", ctr2 * ctr3);

    for i in 0..(lines.len() - 1) {
        for j in (i + 1)..(lines.len()) {
            let temp = almost_equal(&lines[i], &lines[j]);
            if temp.0 {
                println!("res2 = {}", temp.1.unwrap());
                return;
            }
        }
    }

    println!("res2 = ?");
}

fn almost_equal(str1: &str, str2: &str) -> (bool, Option<String>) {
    if str1.len() != str2.len() {
        return (false, None);
    }

    let mut res = String::with_capacity(str1.len());

    let mut diffs = 0u64;
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            diffs += 1;
        } else {
            res.push(c1);
        }

        if diffs > 1 {
            return (false, None);
        }
    }

    if diffs == 0 {
        return (false, None);
    } else {
        return (true, Some(res));
    }
}
