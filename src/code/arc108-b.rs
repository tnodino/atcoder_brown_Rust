// https://atcoder.jp/contests/arc108/tasks/arc108_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        s: String,
    }
    let mut vec = Vec::new();
    let mut M = 0;
    for c in s.chars() {
        vec.push(c);
        M += 1;
        while M >= 3 && vec[M-3] == 'f' && vec[M-2] == 'o' && vec[M-1] == 'x' {
            vec.pop();
            vec.pop();
            vec.pop();
            M -= 3;
        }
    }
    println!("{}", M);
}