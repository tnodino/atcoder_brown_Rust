// https://atcoder.jp/contests/code-festival-2014-morning-easy/tasks/code_festival_morning_easy_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut n: usize,
    }
    n -= 1;
    if n % 40 < 20 {
        println!("{}", n % 40 + 1);
    }
    else {
        println!("{}", 40 - n % 40);
    }
}