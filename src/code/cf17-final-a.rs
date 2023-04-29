// https://atcoder.jp/contests/cf17-final/tasks/cf17_final_a

use proconio::input;
use proconio::fastout;
use regex::Regex;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut S: String,
    }
    let re = Regex::new(r"^A?KIHA?BA?RA?$").unwrap();
    if re.is_match(&S) {
        println!("YES");
    }
    else {
        println!("NO");
    }
}