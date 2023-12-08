use std::{
    cmp::Ordering,
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    hand: String,
    htype: HandType,
    jokers: bool,
}

impl Hand {
    fn new(s: &str, jokers: bool) -> Self {
        let hand = s.to_owned();

        let mut temp = s.chars().collect::<Vec<_>>();
        temp.sort_by(|c1, c2| {
            get_card_strength(c2, jokers)
                .partial_cmp(&get_card_strength(c1, jokers))
                .unwrap()
        });

        let temp = temp
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("");

        let htype = get_hand_type(&temp, jokers);

        Hand {
            hand,
            htype,
            jokers,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let temp = self.htype.partial_cmp(&other.htype).unwrap();
        if temp != Ordering::Equal {
            Some(temp)
        } else {
            for (c1, c2) in self.hand.chars().zip(other.hand.chars()) {
                let temp = get_card_strength(&c1, self.jokers)
                    .partial_cmp(&get_card_strength(&c2, self.jokers))
                    .unwrap();
                if temp != Ordering::Equal {
                    return Some(temp);
                }
            }
            Some(Ordering::Equal)
        }
    }
}

fn get_card_strength(c: &char, jokers: bool) -> i64 {
    match *c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => match jokers {
            true => 1,
            false => 11,
        },
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

fn get_hand_type(s: &str, jokers: bool) -> HandType {
    let mut amounts = HashMap::<char, usize>::new();
    for c in s.chars() {
        amounts.insert(c, s.matches(c).count());
    }

    let mut adder = 0usize;
    if jokers && amounts.get(&'J').is_some() {
        adder += amounts.get(&'J').unwrap();
        amounts.remove(&'J');
    }

    let mut amounts = amounts.into_iter().collect::<Vec<_>>();
    amounts.sort_by(|(.., c1), (.., c2)| c2.partial_cmp(c1).unwrap());

    if jokers {
        if amounts.is_empty() {
            amounts.push(('A', adder));
        } else {
            amounts[0].1 += adder;
        }
    }

    let mut amounts = amounts.into_iter();
    let (.., c1) = amounts.next().unwrap();

    if c1 == 5 {
        HandType::FiveOfAKind
    } else if c1 == 4 {
        HandType::FourOfAKind
    } else if c1 == 3 {
        let (.., c2) = amounts.next().unwrap();
        if c2 == 2 {
            HandType::FullHouse
        } else {
            HandType::ThreeOfAKind
        }
    } else if c1 == 2 {
        let (.., c2) = amounts.next().unwrap();
        if c2 == 2 {
            HandType::TwoPair
        } else {
            HandType::OnePair
        }
    } else {
        HandType::HighCard
    }
}

fn main() {
    let br = BufReader::new(File::open(args().collect::<Vec<_>>().get(1).unwrap()).unwrap());
    let lines = br
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let mut sets = Vec::<(Hand, i64)>::with_capacity(lines.len());
    for line in &lines {
        let mut l = line.split_whitespace();
        let h = Hand::new(l.next().unwrap(), false);
        let a = l.next().unwrap().parse::<i64>().unwrap();
        sets.push((h, a));
    }

    let mut sets_jokers = Vec::<(Hand, i64)>::with_capacity(lines.len());
    for line in &lines {
        let mut l = line.split_whitespace();
        let h = Hand::new(l.next().unwrap(), true);
        let a = l.next().unwrap().parse::<i64>().unwrap();
        sets_jokers.push((h, a));
    }

    let mut res1 = 0i64;
    sets.sort_by(|(h1, ..), (h2, ..)| h1.partial_cmp(h2).unwrap());
    for (i, (.., a)) in sets.iter().enumerate() {
        res1 += (i as i64 + 1) * *a;
    }

    let mut res2 = 0i64;
    sets_jokers.sort_by(|(h1, ..), (h2, ..)| h1.partial_cmp(h2).unwrap());
    for (i, (.., a)) in sets_jokers.iter().enumerate() {
        res2 += (i as i64 + 1) * *a;
    }

    println!("1) {res1}");
    println!("2) {res2}");
}
