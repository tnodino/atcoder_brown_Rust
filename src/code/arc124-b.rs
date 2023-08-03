// https://atcoder.jp/contests/arc124/tasks/arc124_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
        mut b: [usize; N],
    }
    b.sort();
    let mut x = Vec::new();
    for i in 0..N {
        x.push(a[0] ^ b[i]);
    }
    let mut ans = Vec::new();
    for i in 0..N {
        let mut c = Vec::new();
        for j in 0..N {
            c.push(x[i] ^ a[j]);
        }
        c.sort();
        if b == c {
            ans.push(x[i]);
        }
    }
    ans.sort();
    ans.dedup();
    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }
}