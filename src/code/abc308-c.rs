// https://atcoder.jp/contests/abc308/tasks/abc308_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for i in 1..=N {
        input! {
            A: usize,
            B: usize,
        }
        vec.push((A, B, i));
    }
    vec.sort_by(|a, b|
    if a.0 * (b.0 + b.1) == b.0 * (a.0 + a.1) {
        a.2.cmp(&b.2)
    }
    else {
        (b.0 * (a.0 + a.1)).cmp(&(a.0 * (b.0 + b.1)))
    });
    println!("{}", vec.iter().map(|&x| x.2.to_string()).collect::<Vec<String>>().join(" "));
}