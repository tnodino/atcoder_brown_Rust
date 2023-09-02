// https://atcoder.jp/contests/arc142/tasks/arc142_a

use proconio::input;
use proconio::fastout;

fn rev(k: usize) -> usize {
    return k.to_string().chars().rev().collect::<String>().parse::<usize>().unwrap();
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut vec = Vec::new();
    if rev(rev(K)) != K || rev(K) < K {
        println!("0");
        return;
    }
    let mut x = K;
    while x <= N {
        if rev(rev(x)) == K {
            vec.push(x);
        }
        x *= 10;
    }
    let mut x = rev(K);
    while x <= N {
        if rev(x) == K {
            vec.push(x);
        }
        x *= 10;
    }
    vec.sort();
    vec.dedup();
    println!("{}", vec.len());
}