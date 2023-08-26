// https://atcoder.jp/contests/abc278/tasks/abc278_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        Q: usize,
    }
    let mut vec = (0..N).collect::<Vec<usize>>();
    let mut s = 0;
    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: usize,
            }
            while !vec.is_empty() {
                let idx = vec.pop().unwrap();
                A[idx] = 0;
            }
            s = x;
        }
        else if q == 2 {
            input! {
                i: usize,
                x: usize,
            }
            A[i-1] += x;
            vec.push(i-1);
        }
        else {
            input! {
                i: usize,
            }
            println!("{}", A[i-1] + s);
        }
    }
}