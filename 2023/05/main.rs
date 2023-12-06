use std::{
    cmp::max,
    cmp::min,
    collections::HashMap,
    collections::VecDeque,
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

    let seeds = lines
        .first()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut res1n = seeds.clone();
    let mut kinds = Vec::<&str>::new();
    let mut maps = HashMap::<(&str, &str), Vec<(i64, i64, i64)>>::new();

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
            res.push((
                lniter.next().unwrap(),
                lniter.next().unwrap(),
                lniter.next().unwrap(),
            ));
        }
    }

    for kind in &kinds {
        for (k, v) in &maps {
            if k.0 == *kind {
                for n in res1n.iter_mut() {
                    let mut new_n: Option<i64> = None;
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

    let mut res2n = Vec::<(i64, i64)>::new();

    for i in (0..seeds.len()).step_by(2) {
        res2n.push((*seeds.get(i).unwrap(), *seeds.get(i + 1).unwrap()));
    }

    for kind in &kinds {
        for (k, v) in &maps {
            if k.0 == *kind {
                let mut temp = Vec::new();

                for r in res2n.iter() {
                    let res = remap_range(r, v);
                    temp.extend(res);
                }

                res2n = combine_ranges(&temp);
                break;
            }
        }
    }

    res2n.sort_by(|(v1, ..), (v2, ..)| v1.partial_cmp(v2).unwrap());
    let res2 = res2n.first().unwrap().0;
    println!("2) {res2}");
}

fn combine_ranges(a: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut perm = true;
    let mut res = a.to_owned();

    'first: while perm {
        for i in 0..res.len() {
            for j in (i + 1)..res.len() {
                let alb = min(res[i].0, res[j].0);
                let ahb = max(res[i].0 + res[i].1 - 1, res[j].0 + res[j].1 - 1);
                let llb = max(res[i].0, res[j].0);
                let lhb = min(res[i].0 + res[i].1 - 1, res[j].0 + res[j].1 - 1);

                if llb - 1 <= lhb {
                    res.remove(i);
                    res.remove(j - 1);
                    res.push((alb, ahb - alb + 1));
                    perm = true;
                    continue 'first;
                }
            }
        }
        perm = false;
    }

    res
}

fn remap_range(f: &(i64, i64), t: &[(i64, i64, i64)]) -> Vec<(i64, i64)> {
    let mut um = VecDeque::<(i64, i64)>::from([f.to_owned()]);
    let mut num = VecDeque::<(i64, i64)>::new();
    let mut m = VecDeque::<(i64, i64)>::new();
    let nr = t.to_owned();

    for r in nr.iter() {
        while let Some(remapping) = um.pop_front() {
            let lb = max(remapping.0, r.1);
            let hb = min(remapping.0 + remapping.1 - 1, r.1 + r.2 - 1);

            if lb > hb {
                num.push_back(remapping);
                continue;
            }

            m.push_back((lb + (r.0 - r.1), hb - lb + 1));

            if lb > remapping.0 {
                num.push_back((remapping.0, lb - remapping.0));
            }

            if hb < remapping.0 + remapping.1 - 1 {
                num.push_back((hb + 1, remapping.0 + remapping.1 - 1 - hb));
            }
        }
        um.extend(&num);
        num.clear();
        um = combine_ranges(&um.into_iter().collect::<Vec<_>>()).into();
    }

    um.extend(m);
    combine_ranges(&um.into_iter().collect::<Vec<_>>())
}
