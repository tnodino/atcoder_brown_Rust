// https://atcoder.jp/contests/code-formula-2014-qualb/tasks/code_formula_2014_qualB_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    let N = N.chars().rev().collect::<Vec<char>>();
    let mut odd = 0;
    let mut even = 0;
    for (i, n) in N.iter().enumerate() {
        if i % 2 == 1 {
            odd += *n as usize - 48;
        }
        else {
            even += *n as usize - 48;
        }
    }
    println!("{} {}", odd, even);
}