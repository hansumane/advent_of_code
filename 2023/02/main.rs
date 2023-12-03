use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let br = BufReader::new(File::open(args().collect::<Vec<_>>().get(1).unwrap()).unwrap());
    let max_values: HashMap<&str, u64> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut res1 = 0u64;
    let mut res2 = 0u64;

    for (li, line) in br.lines().enumerate() {
        match line {
            Ok(line) => {
                let mut possible = true;

                'first: for game in line.split(": ").last().unwrap().split("; ") {
                    for cubes in game.split(", ") {
                        let mut cubsplit = cubes.split(' ');
                        let (amount, color): (u64, &str) = (
                            cubsplit.next().unwrap().parse().unwrap(),
                            cubsplit.next().unwrap(),
                        );
                        if amount > max_values[color] {
                            possible = false;
                            break 'first;
                        }
                    }
                }

                if possible {
                    res1 += li as u64 + 1;
                }

                let mut mins: HashMap<&str, u64> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

                for game in line.split(": ").last().unwrap().split("; ") {
                    for cubes in game.split(", ") {
                        let mut cubsplit = cubes.split(' ');
                        let (amount, color): (u64, &str) = (
                            cubsplit.next().unwrap().parse().unwrap(),
                            cubsplit.next().unwrap(),
                        );
                        if mins[color] < amount {
                            mins.insert(color, amount);
                        }
                    }
                }

                res2 += mins.into_iter().reduce(|(k, v1), (.., v2)| (k, v1 * v2)).unwrap().1;
            }
            Err(_) => break,
        }
    }

    println!("1) {res1}");
    println!("2) {res2}");
}
