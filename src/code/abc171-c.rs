// https://atcoder.jp/contests/abc171/tasks/abc171_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: u64,
    }
    let mut ans = "".to_string();
    while N > 0 {
        N -= 1;
        ans = format!("{}{}", ans, ((N % 26) as u8 + 97) as char);
        N /= 26;
    }
    println!("{}", ans.chars().rev().collect::<String>());
}