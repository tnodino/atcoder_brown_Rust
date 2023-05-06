// https://atcoder.jp/contests/abc174/tasks/abc174_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        c: String,
    }
    let c = c.chars().collect::<Vec<char>>();
    let cnt = c.iter().filter(|&x| *x == 'R').count();
    let mut ans = 0;
    for i in 0..cnt {
        if c[i] == 'W' {
            ans += 1;
        }
    }
    println!("{}", ans);
}