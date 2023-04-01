// https://atcoder.jp/contests/abc019/tasks/abc019_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    let N = s.len();
    let s = s.chars().collect::<Vec<char>>();
    let mut c = s[0];
    let mut cnt = 1;
    for i in 1..N {
        if s[i] == c {
            cnt += 1;
        }
        else {
            print!("{}{}", c, cnt);
            c = s[i];
            cnt = 1;
        }
    }
    print!("{}{}", c, cnt);
    println!();
}