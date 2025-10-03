use std::collections::HashSet;
use std::{fs::File, io::{BufReader, Read}};

fn main() {
    let file = File::open("input.txt").expect("input.txt");
    let br = BufReader::new(file);

    let mut res1: HashSet<(i64, i64)> = HashSet::from([(0, 0)]);
    let mut i1: i64 = 0;
    let mut j1: i64 = 0;

    let mut res2: HashSet<(i64, i64)> = HashSet::from([(0, 0)]);
    let mut i2: [i64; 2] = [0, 0];
    let mut j2: [i64; 2] = [0, 0];

    for (i, direction) in br.bytes().enumerate() {
        let Ok(symbol) = direction else { break };
        let symbol = symbol as char;

        match symbol {
            '^' => {
                i1 += 1;
                i2[i % 2] += 1;
            }
            'v' => {
                i1 -= 1;
                i2[i % 2] -= 1;
            }
            '<' => {
                j1 -= 1;
                j2[i % 2] -= 1;
            }
            '>' => {
                j1 += 1;
                j2[i % 2] += 1;
            }
            _ => {} // ignore any other symbols
        }

        res1.insert((i1, j1));
        res2.insert((i2[0], j2[0]));
        res2.insert((i2[1], j2[1]));
    }

    println!("amount of houses (1st part): {}", res1.len());
    println!("amount of houses (2nd part): {}", res2.len());
}
