use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let br = BufReader::new(File::open(args().collect::<Vec<_>>().get(1).unwrap()).unwrap());
    let lines = br.lines().filter(|line| line.is_ok()).map(|line| line.unwrap()).collect::<Vec<_>>();
    let mll = lines.get(0).unwrap().len();

    let mut numbers = Vec::<(usize, u64, usize, usize)>::new();
    let mut gears = Vec::<(usize, usize)>::new();

    for (li, line) in lines.iter().enumerate() {
        let mut i = 0usize;
        let chars = line.chars().collect::<Vec<_>>();

        while i < line.len() {
            let s = chars.get(i).unwrap();
            match s {
                '0'..='9' => {
                    let si = i;
                    let mut ei = i + 1;
                    let mut n = s.to_string().parse::<u64>().unwrap();

                    while ei < line.len() {
                        let is = chars.get(ei).unwrap();
                        match is {
                            '0'..='9' => {
                                n = n * 10 + is.to_string().parse::<u64>().unwrap();
                                ei += 1;
                            }
                            _ => break,
                        }
                    }

                    i = ei;
                    ei -= 1;
                    numbers.push((li, n, si, ei));
                }
                _ => i += 1,
            }
        }

        for (i, s) in chars.iter().enumerate() {
            if *s == '*' {
                gears.push((li, i));
            }
        }
    }

    let mut res1 = 0u64;

    for num in numbers.iter() {
        let (mut above, mut cur, mut below) = (false, false, false);
        let (li, n, mut si, mut ei) = *num;

        if si > 0 {
            si -= 1;
            match lines.get(li).unwrap().chars().collect::<Vec<_>>().get(si).unwrap() {
                '@' | '#' | '$' | '%' | '&' | '*' | '-' | '=' | '+' | '/' => cur = true,
                _ => {}
            }
        }

        if ei < mll - 1 {
            ei += 1;
            match lines.get(li).unwrap().chars().collect::<Vec<_>>().get(ei).unwrap() {
                '@' | '#' | '$' | '%' | '&' | '*' | '-' | '=' | '+' | '/' => cur = true,
                _ => {}
            }
        }

        if li > 0 {
            let chars = lines.get(li - 1).unwrap().chars().collect::<Vec<_>>();
            for i in si..=ei {
                match chars.get(i).unwrap() {
                    '@' | '#' | '$' | '%' | '&' | '*' | '-' | '=' | '+' | '/' => {
                        above = true;
                        break;
                    }
                    _ => {}
                }
            }
        }

        if li < lines.len() - 1 {
            let chars = lines.get(li + 1).unwrap().chars().collect::<Vec<_>>();
            for i in si..=ei {
                match chars.get(i).unwrap() {
                    '@' | '#' | '$' | '%' | '&' | '*' | '-' | '=' | '+' | '/' => {
                        below = true;
                        break;
                    }
                    _ => {}
                }
            }
        }

        if above || cur || below {
            res1 += n;
        }
    }

    let mut res2 = 0u64;

    for gear in gears.iter() {
        let (li, gi) = *gear;
        let mut ns = Vec::<u64>::new();

        for num in numbers.iter().filter(|(nli, ..)| *nli == li) {
            let (_, n, si, ei) = *num;
            if gi > 0 && ei == gi - 1 {
                ns.push(n);
            }
            if si == gi + 1 {
                ns.push(n);
            }
        }

        if li > 0 {
            for num in numbers.iter().filter(|(nli, ..)| *nli == li - 1) {
                let (_, n, mut si, mut ei) = *num;
                si = si.saturating_sub(1);

                if ei < mll - 1 {
                    ei += 1;
                }

                if si <= gi && gi <= ei {
                    ns.push(n);
                }
            }
        }

        if li < lines.len() - 1 {
            for num in numbers.iter().filter(|(nli, ..)| *nli == li + 1) {
                let (_, n, mut si, mut ei) = *num;
                si = si.saturating_sub(1);

                if ei < mll - 1 {
                    ei += 1;
                }

                if si <= gi && gi <= ei {
                    ns.push(n);
                }
            }
        }

        if ns.len() == 2 {
            res2 += ns.first().unwrap() * ns.get(1).unwrap();
        }
    }

    println!("1) {res1}");
    println!("2) {res2}");
}
