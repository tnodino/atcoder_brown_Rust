// https://atcoder.jp/contests/arc142/tasks/arc142_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = vec![vec![0; N]; N];
    let mut x = 1;
    for i in (0..N).step_by(2) {
        for j in 0..N {
            ans[i][j] = x;
            x += 1;
        }
    }
    for i in (1..N).step_by(2) {
        for j in 0..N {
            ans[i][j] = x;
            x += 1;
        }
    }
    for i in 0..N {
        println!("{}", ans[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}