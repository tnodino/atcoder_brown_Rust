// https://atcoder.jp/contests/abc319/tasks/abc319_d

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn f(x: usize, N: usize, M: usize, L: &Vec<usize>) -> bool {
    if *L.iter().max().unwrap() > x {
        return false;
    }
    let mut cnt = 0;
    let mut row = 1;
    for i in 0..N {
        if cnt + L[i] > x {
            row += 1;
            cnt = 0;
        }
        cnt += L[i];
        match i + 1 < N && cnt + 1 >= x {
            true => {
                row += 1;
                cnt = 0;
            },
            false => cnt += 1,
        }
    }
    return row <= M;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        L: [usize; N],
    }
    let mut ok = L.iter().sum::<usize>() + N - 1;
    let mut ng = 0;
    for _ in 0..100 {
        let mid = (ok + ng) / 2;
        match f(mid, N, M, &L) {
            true => ok = mid,
            false => ng = mid,
        };
    }
    println!("{}", ok);
}