// https://atcoder.jp/contests/indeednow-qualb/tasks/indeednow_2015_qualb_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let N = s.len();
    let mut s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();
    for i in 0..N {
        if s == t {
            println!("{}", i);
            return;
        }
        let mut vec = vec![s[N-1]];
        vec.extend(s[..N-1].iter());
        s = vec;
    }
    println!("-1");
}