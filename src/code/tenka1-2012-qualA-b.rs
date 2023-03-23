// https://atcoder.jp/contests/tenka1-2012-qualA/tasks/tenka1_2012_qualA_2

use proconio::input;
use proconio::is_stdin_empty;

#[allow(non_snake_case)]
fn main() {
    let mut ans = Vec::new();
    loop {
        if is_stdin_empty() {
            break;
        }
        input! {
            c: String,
        }
        ans.push(c);
    }
    println!("{}", ans.join(","));
}