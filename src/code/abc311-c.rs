// https://atcoder.jp/contests/abc311/tasks/abc311_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut flg = vec![false; N];
    let mut vec = Vec::new();
    let mut idx = 1;
    while !flg[idx-1] {
        flg[idx-1] = true;
        vec.push(idx);
        idx = A[idx-1];
    }
    let idx = vec.iter().position(|&x| x == idx).unwrap();
    let ans = &vec[idx..];
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}