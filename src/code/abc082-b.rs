// https://atcoder.jp/contests/abc082/tasks/abc082_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let mut s = s.chars().collect::<Vec<char>>();
    let mut t = t.chars().collect::<Vec<char>>();
    s.sort_by(|a, b| a.cmp(b));
    t.sort_by(|a, b| b.cmp(a));
    if s < t {
        println!("Yes");
    }
    else {
        println!("No");
    }
}