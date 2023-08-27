// https://atcoder.jp/contests/abc314/tasks/abc314_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
        Q: usize,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut T = Vec::new();
    for i in 0..N {
        T.push((0, S[i]));
    }
    let mut last = (0, 0);
    for i in 1..=Q {
        input! {
            t: usize,
            x: usize,
            c: char,
        }
        if t == 1 {
            T[x-1] = (i, c);
        }
        else {
            last = (i, t);
        }
    }
    for i in 0..N {
        if T[i].0 > last.0 || last.0 == 0 {
            print!("{}", T[i].1);
        }
        else {
            if last.1 == 2 {
                print!("{}", T[i].1.to_lowercase());
            }
            else {
                print!("{}", T[i].1.to_uppercase());
            }
        }
    }
    println!();
}