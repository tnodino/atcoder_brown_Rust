// https://atcoder.jp/contests/abc047/tasks/arc063_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    S.dedup();
    println!("{:?}", S.len() - 1);
}