// https://atcoder.jp/contests/abc250/tasks/abc250_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut ans = (0..N).collect::<Vec<usize>>();
    let mut pos = (0..N).collect::<Vec<usize>>();
    for _ in 0..Q {
        input! {
            x: usize,
        }
        let a1 = pos[x-1];
        let a2;
        if a1 == N - 1 {
            a2 = a1 - 1;
        }
        else {
            a2 = a1 + 1;
        }
        let p1 = ans[a1];
        let p2 = ans[a2];
        ans.swap(a1, a2);
        pos.swap(p1, p2);
    }
    println!("{}", ans.iter().map(|&x| (x+1).to_string()).collect::<Vec<String>>().join(" "));
}