use std::{
    collections::HashMap,
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

    let lrs: &str = lines.first().unwrap();
    let mut rules = HashMap::<&str, (&str, &str)>::new();
    let mut current = "AAA";

    for line in &lines[2..] {
        let mut ls = line.split(" = ");
        let lhs = ls.next().unwrap();
        let rhs = ls.next().unwrap();
        let rhs = &rhs[1..(rhs.len() - 1)];

        let mut rhss = rhs.split(", ");
        let rhsl = rhss.next().unwrap();
        let rhsr = rhss.next().unwrap();

        rules.insert(lhs, (rhsl, rhsr));
    }

    let mut res1 = 0usize;
    while current != "ZZZ" {
        let dir = lrs.chars().nth(res1 % lrs.len()).unwrap();
        match dir {
            'L' => current = rules.get(current).unwrap().0,
            'R' => current = rules.get(current).unwrap().1,
            _ => panic!(),
        }
        res1 += 1;
    }

    let mut currents = Vec::<(String, usize)>::new();
    for k in rules.keys() {
        if k.chars().nth(k.len() - 1).unwrap() == 'A' {
            currents.push((k.to_string(), 0));
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for (n, a) in currents.iter_mut() {
            if !n.ends_with('Z') {
                let dir = lrs.chars().nth(*a % lrs.len()).unwrap();
                match dir {
                    'L' => *n = rules.get(n.as_str()).unwrap().0.to_string(),
                    'R' => *n = rules.get(n.as_str()).unwrap().1.to_string(),
                    _ => panic!(),
                }
                *a += 1;
                changed = true;
            }
        }
    }

    let mut res2 = 1usize;
    for (.., a) in currents {
        res2 = lcm(res2, a);
    }

    println!("1) {res1}");
    println!("2) {res2}");
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = std::cmp::max(first, second);
    let mut min = std::cmp::min(first, second);

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}
