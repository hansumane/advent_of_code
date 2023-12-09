use std::{
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

    let values = lines
        .iter()
        .map(|v| {
            v.split_whitespace()
                .map(|l| l.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res1 = 0i64;
    let mut res2 = 0i64;

    for value in &values {
        let mut stop = false;
        let mut subs = Vec::<VecDeque<i64>>::new();
        subs.push(value.clone().into());

        while !stop {
            let mut sub = VecDeque::<i64>::new();
            for i in 1..subs.last().unwrap().len() {
                sub.push_back(subs.last().unwrap()[i] - subs.last().unwrap()[i - 1]);
            }
            if sub.iter().all(|v| v == &0) {
                stop = true;
            }
            subs.push(sub);
        }

        subs.last_mut().unwrap().push_back(0);
        subs.last_mut().unwrap().push_front(0);
        for i in (0..(subs.len() - 1)).rev() {
            let mut temp = subs.get(i + 1).unwrap().iter();
            let a = temp.next().unwrap().to_owned();
            let b = temp.last().unwrap().to_owned();

            let s = subs.get_mut(i).unwrap();
            s.push_back(s.iter().last().unwrap() + b);
            s.push_front(s.iter().next().unwrap() - a);
        }

        let temp = subs.first().unwrap().iter().collect::<Vec<_>>();
        res1 += *temp.last().unwrap();
        res2 += *temp.first().unwrap();
    }

    println!("1) {res1}");
    println!("2) {res2}");
}
