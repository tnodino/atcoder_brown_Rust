// https://atcoder.jp/contests/abc043/tasks/abc043_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    let mut ans = Vec::new();
    for c in s.chars() {
        match c {
            '0' => ans.push('0'),
            '1' => ans.push('1'),
            _ => {
                if ans.is_empty() {
                    continue;
                }
                ans.pop();
            },
        }
    }
    println!("{}", ans.iter().collect::<String>());
}