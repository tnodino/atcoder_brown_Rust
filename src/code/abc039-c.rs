// https://atcoder.jp/contests/abc039/tasks/abc039_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.replace("WB", "X");
    let mut idx = Vec::new();
    for (i, s) in S.chars().enumerate() {
        if s == 'W' {
            idx.push(i);
        }
    }
    println!("{}", match (idx[0], idx[1]) {
        (2, 6) => "Do",
        (1, 5) => "Re",
        (0, 4) => "Mi",
        (3, 6) => "Fa",
        (2, 5) => "So",
        (1, 4) => "La",
        (_, _) => "Si",
    });
}