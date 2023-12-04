use std::{
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

    let mut cards: Vec<(usize, Vec<u64>, Vec<u64>, u64)> = Vec::with_capacity(lines.len());

    for (li, line) in lines.iter().enumerate() {
        let mut game = line.split(": ").last().unwrap().split(" | ");
        let win_cards = game
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let all_cards = game
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        cards.push((li + 1, win_cards, all_cards, 0));
    }

    let mut res1 = 0u64;

    for (_, win_cards, all_cards, points) in &mut cards {
        let mut p = 0u64;
        let mut a = 0u64;
        for card in all_cards {
            if win_cards.contains(card) {
                p = match p {
                    0 => p + 1,
                    _ => p * 2,
                };
                a += 1;
            }
        }

        *points = a;
        res1 += p;
    }

    let mut ci = 0usize;

    /*
     * This is a very bad solution because here we allocate around 8 million cards
     * which must take a HUGE amount of memory and for sure is VERY SLOW
     * yet it works tho :')
     */

    while ci < cards.len() {
        let mut i = cards.get(ci).unwrap().0;
        let a = cards.get(ci).unwrap().3;

        for _ in 0..a {
            cards.push(cards.get(i).unwrap().clone());
            i += 1;
        }

        ci += 1;
    }

    println!("1) {}", res1);
    println!("2) {}", cards.len());
}
