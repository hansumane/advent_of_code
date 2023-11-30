use std::fs::File;
use std::io::prelude::*;

fn read_file_contents(filename: &str) -> Option<String> {
    let mut file;

    if let Ok(f) = File::open(filename) {
        file = f;
    } else {
        return None;
    }

    let mut c = String::new();

    if let Ok(_) = file.read_to_string(&mut c) {
        return Some(c.trim().to_string());
    } else {
        return None;
    }
}

fn main() {
    let filename = "input.txt".to_string();
    let text = read_file_contents(&filename).unwrap();

    /* first */

    let mut floor = 0i64;

    for c in text.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }

    println!("result floor: {}", floor);

    /* second */

    let mut floor = 0i64;
    let mut ctr = 1u64;

    for c in text.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor < 0 {
            break;
        }
        ctr += 1;
    }

    println!("floor: {}", floor);
    println!("basement: {}", ctr);
}
