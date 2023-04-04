// https://atcoder.jp/contests/dwango2016-prelims/tasks/dwango2016qual_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: [usize; N-1],
    }
    let mut ans = vec![K[0]];
    for i in 0..N-2 {
        ans.push(min(K[i], K[i+1]))
    }
    ans.push(K[N-2]);
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}