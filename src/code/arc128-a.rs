// https://atcoder.jp/contests/arc128/tasks/arc128_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = vec![0; N];
    for i in 0..N-1 {
        if A[i] > A[i+1] {
            ans[i] ^= 1;
            ans[i+1] ^= 1;
        }
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}