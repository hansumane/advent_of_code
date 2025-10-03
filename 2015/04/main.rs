extern crate md5;

use md5::{Md5, Digest};

fn main() {
    let mut found5 = false;
    let mut found6 = false;

    for i in 0.. {
        let mut hasher = Md5::new();
        hasher.update(format!("iwrupvqb{}", i));

        let res = hasher.finalize();
        let res = format!("{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            res[ 0], res[ 1], res[ 2], res[ 3],
            res[ 4], res[ 5], res[ 6], res[ 7],
            res[ 8], res[ 9], res[10], res[11],
            res[12], res[13], res[14], res[15]
        );

        if !found5 && res.starts_with("00000") {
            found5 = true;
            println!("5. {}, {}", res, i);
        }

        if !found6 && res.starts_with("000000") {
            found6 = true;
            println!("6. {}, {}", res, i);
        }

        if found5 && found6 {
            break;
        }
    }
}
