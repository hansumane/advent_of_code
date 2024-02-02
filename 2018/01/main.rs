use std::{collections::HashSet, env::args, fs::read_to_string};

fn main() {
    let s = read_to_string(args().collect::<Vec<_>>().get(1).unwrap()).unwrap();
    let l: Vec<&str> = s.lines().collect();
    let n: Vec<i64> = l.iter().map(|v| v.parse().unwrap()).collect();
    let res1: i64 = n.iter().sum();

    let mut set = HashSet::<i64>::new();
    set.insert(0);

    let mut res2 = 0;

    'outer: loop {
        for v in n.iter() {
            res2 += *v;
            if set.contains(&res2) {
                break 'outer;
            }
            set.insert(res2);
        }
    }

    println!("{}", res1);
    println!("{}", res2);
}
