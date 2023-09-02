// https://atcoder.jp/contests/abc232/tasks/abc232_c

use proconio::input;
use itertools::Itertools;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut flg1 = vec![vec![false; N]; N];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        flg1[A-1][B-1] = true;
        flg1[B-1][A-1] = true;
    }
    let mut flg2 = vec![vec![false; N]; N];
    for _ in 0..M {
        input! {
            C: usize,
            D: usize,
        }
        flg2[C-1][D-1] = true;
        flg2[D-1][C-1] = true;
    }
    'outer: for perm in (0..N).permutations(N) {
        for i in 0..N {
            for j in 0..N {
                if flg1[i][j] != flg2[perm[i]][perm[j]] {
                    continue 'outer;
                }
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}