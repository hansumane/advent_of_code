use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("input.txt").unwrap();
    let br = BufReader::new(file);

    let mut paper = 0u64;
    let mut ribbon = 0u64;

    for line in br.lines() {
        if let Ok(line) = line {
            let mut sizes = line.split('x')
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>();
            sizes.sort();

            let x = sizes[0];
            let y = sizes[1];
            let z = sizes[2];

            let rp = vec![x*y, x*z, y*z];

            paper += 2 * rp.iter().sum::<u64>();
            paper += rp[0];

            ribbon += x * y * z;
            ribbon += 2 * sizes[0] + 2 * sizes[1];

        } else {
            break;
        }
    }

    println!("paper: {}", paper);
    println!("ribbon: {}", ribbon);
}
