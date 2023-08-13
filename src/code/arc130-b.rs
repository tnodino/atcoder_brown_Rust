// https://atcoder.jp/contests/arc130/tasks/arc130_b

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        C: usize,
        Q: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..Q {
        input! {
            t: usize,
            n: usize,
            c: usize,
        }
        vec.push((t, n, c-1));
    }
    let mut setr = HashSet::new();
    let mut setc = HashSet::new();
    let mut ans = vec![0; C];
    for i in (0..Q).rev() {
        if vec[i].0 == 1 {
            if setr.contains(&vec[i].1) {
                continue;
            }
            setr.insert(vec[i].1);
            ans[vec[i].2] += W - setc.len();
        }
        else {
            if setc.contains(&vec[i].1) {
                continue;
            }
            setc.insert(vec[i].1);
            ans[vec[i].2] += H - setr.len();
        }
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}