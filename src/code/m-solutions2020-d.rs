// https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut money = 1000;
    let mut cnt = 0;
    for i in 0..N-1 {
        if A[i] < A[i+1] {
            let x = money / A[i];
            money -= A[i] * x;
            cnt += x;
        }
        else {
            money += A[i] * cnt;
            cnt = 0;
        }
    }
    money += A[N-1] * cnt;
    println!("{}", money);
}