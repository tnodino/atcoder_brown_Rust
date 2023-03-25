// https://atcoder.jp/contests/abc004/tasks/abc004_3

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    N %= 30;
    let mut ans = [1, 2, 3, 4, 5, 6];
    for i in 0..N {
        let idx = i % 5;
        ans.swap(idx, idx + 1);
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<String>());
}