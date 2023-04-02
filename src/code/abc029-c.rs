// https://atcoder.jp/contests/abc029/tasks/abc029_c

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for bit in 0..pow(3, N) {
        let mut vec = Vec::new();
        let mut x = bit;
        for _ in 0..N {
            vec.push(((x % 3) as u8 + 97) as char);
            x /= 3;
        }
        println!("{}", vec.iter().rev().collect::<String>());
    }
}