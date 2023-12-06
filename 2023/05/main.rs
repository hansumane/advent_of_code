use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

fn main() {
    let br = BufReader::new(File::open(args().collect::<Vec<_>>().get(1).unwrap()).unwrap());
    let lines = br
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let seeds = lines
        .first()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut res1n = seeds.clone();
    let mut kinds = Vec::<&str>::new();
    let mut maps = HashMap::<(&str, &str), Vec<(u64, u64, u64)>>::new();

    let mut liter = lines.iter();
    liter.next(); // skip seeds and next empty line
    liter.next();

    while let Some(line) = liter.next() {
        let map_names = line.split_whitespace().next().unwrap().split("-to-").collect::<Vec<&str>>();
        let mnlhs = map_names.first().unwrap();
        let mnrhs = map_names.last().unwrap();

        kinds.push(mnlhs);
        maps.insert((mnlhs, mnrhs), Vec::new());
        let res = maps.get_mut(&(mnlhs, mnrhs)).unwrap();

        for line in liter.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut lniter = line.split_whitespace().map(|v| v.parse().unwrap());
            res.push((lniter.next().unwrap(), lniter.next().unwrap(), lniter.next().unwrap()));
        }
    }

    for kind in &kinds {
        for (k, v) in &maps {
            if k.0 == *kind {
                for n in res1n.iter_mut() {
                    let mut new_n: Option<u64> = None;
                    for r in v {
                        if r.1 <= *n && *n < r.1 + r.2 {
                            new_n = Some(*n - r.1 + r.0);
                            break;
                        }
                    }
                    if let Some(new_n) = new_n {
                        *n = new_n;
                    }
                }
                break;
            }
        }
    }

    println!("1) {res1}", res1 = res1n.iter().min().unwrap());

    /*
     * well, the second part works,
     * just start it and take a coffee break
     * or smoke a couple of cigarettes
     * because that's definitely a smoker's solution
     */

    let mut res2 = u64::MAX;
    let mut res2n = Vec::<(u64, u64)>::new();

    for i in (0..seeds.len()).step_by(2) {
        res2n.push((*seeds.get(i).unwrap(), *seeds.get(i + 1).unwrap()));
    }

    for v in res2n {
        let mut s = v.0;
        let a = v.1;

        for _ in 0..a {
            let mut n = s;

            for kind in &kinds {
                for (k, v) in &maps {
                    if k.0 == *kind {
                        let mut new_n = Option::<u64>::None;
                        for r in v {
                            if r.1 <= n && n < r.1 + r.2 {
                                new_n = Some(n - r.1 + r.0);
                                break;
                            }
                        }
                        if let Some(new_n) = new_n {
                            n = new_n;
                        }
                        break;
                    }
                }
            }

            if n < res2 {
                res2 = n;
                println!("current min: {res2}");
            }

            s += 1;
        }
    }

    println!("2) {res2}");
}
