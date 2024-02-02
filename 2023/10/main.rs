use std::{
    collections::{HashMap, VecDeque},
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
    let mi = lines.len() as i64;
    let mj = lines.first().unwrap().len() as i64;

    let mut rules = HashMap::<(i64, i64), ((i64, i64), (i64, i64))>::new();
    let mut start = (mi, mj);

    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            let i = i as i64;
            let j = j as i64;
            match c {
                '|' => rules.insert((i, j), ((i-1, j), (i+1, j))),
                '-' => rules.insert((i, j), ((i, j-1), (i, j+1))),
                'F' => rules.insert((i, j), ((i, j+1), (i+1, j))),
                '7' => rules.insert((i, j), ((i, j-1), (i+1, j))),
                'L' => rules.insert((i, j), ((i-1, j), (i, j+1))),
                'J' => rules.insert((i, j), ((i-1, j), (i, j-1))),
                'S' => {
                    start = (i, j);
                    let mut toap = Vec::<(i64, i64)>::new();
                    if i > 0 {
                        let tc = lines.get(i as usize - 1).unwrap().chars().nth(j as usize).unwrap();
                        if tc == '7' || tc == '|' || tc == 'F' {
                            toap.push((i - 1, j));
                        }
                    }
                    if j > 0 {
                        let tc = lines.get(i as usize).unwrap().chars().nth(j as usize - 1).unwrap();
                        if tc == 'L' || tc == '-' || tc == 'F' {
                            toap.push((i, j - 1));
                        }
                    }
                    if i < mi - 1 {
                        let tc = lines.get(i as usize + 1).unwrap().chars().nth(j as usize).unwrap();
                        if tc == 'J' || tc == '|' || tc == 'L' {
                            toap.push((i + 1, j));
                        }
                    }
                    if j < mj - 1 {
                        let tc = lines.get(i as usize).unwrap().chars().nth(j as usize + 1).unwrap();
                        if tc == 'J' || tc == '-' || tc == '7' {
                            toap.push((i, j + 1));
                        }
                    }
                    let mut toapi = toap.into_iter();
                    rules.insert((i, j), (toapi.next().unwrap(), toapi.next().unwrap()))
                },
                _ => None,
            };
        }
    }

    let mut path = VecDeque::<(i64, i64)>::new();
    let mut current = start;
    let mut prev = (mi, mj);

    loop {
        let a = rules.get(&current).unwrap().to_owned();
        path.push_back(current);

        if a.0 != prev {
            prev = current;
            current = a.0;
        } else {
            prev = current;
            current = a.1;
        }

        if current == start {
            break;
        }
    }

    let res1 = path.len() / 2;

    /*
     * for part 2 everything we basically need is to find
     * an area enclosed by the path we got in part 1
     */

    let mut path2 = path.clone();
    let temp = path2.pop_front().unwrap();
    path2.push_back(temp);

    let mut res2 = 0i64;
    for ((i0, j0), (i1, j1)) in path.iter().zip(path2.iter()) {
        res2 += j0 * i1 - i0 * j1;
    }
    res2 = (res2.abs() - path.len() as i64) / 2 + 1;

    println!("1) {res1}");
    println!("2) {res2}");
}
