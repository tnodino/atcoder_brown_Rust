// https://atcoder.jp/contests/abc171/tasks/abc171_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut a: [usize; N],
    }
    let mut xor = 0;
    for i in 0..N {
        xor ^= a[i];
    }
    for i in 0..N {
        a[i] ^= xor;
    }
    println!("{}", a.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}