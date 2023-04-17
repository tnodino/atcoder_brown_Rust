// https://atcoder.jp/contests/agc013/tasks/agc013_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.dedup();
    let N = A.len();
    let mut ans = 1;
    let mut flg = '?';
    for i in 0..N-1 {
        if flg == '<' {
            if A[i] > A[i+1] {
                ans += 1;
                flg = '?';
            }
        }
        else if flg == '>' {
            if A[i] < A[i+1] {
                ans += 1;
                flg = '?';
            }
        }
        else {
            if A[i] < A[i+1] {
                flg = '<';
            }
            else {
                flg = '>';
            }
        }
    }
    println!("{}", ans);
}