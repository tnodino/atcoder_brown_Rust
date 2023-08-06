// https://atcoder.jp/contests/abc225/tasks/abc225_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut front = vec![-1; N+1];
    let mut back = vec![-1; N+1];
    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: usize,
                y: usize,
            }
            front[y] = x as isize;
            back[x] = y as isize;
        }
        else if q == 2 {
            input! {
                x: usize,
                y: usize,
            }
            front[y] = -1;
            back[x] = -1;
        }
        else {
            input! {
                mut x: usize,
            }
            while front[x] != -1 {
                x = front[x] as usize;
            }
            let mut ans = vec![0, x];
            while back[x] != -1 {
                x = back[x] as usize;
                ans.push(x);
            }
            ans[0] = ans.len() - 1;
            println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
    }
}