// https://atcoder.jp/contests/abc131/tasks/abc131_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        vec.push((A, B));
    }
    vec.sort_by(|a, b| a.1.cmp(&b.1));
    let mut time = 0;
    for i in 0..N {
        time += vec[i].0;
        if time > vec[i].1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}