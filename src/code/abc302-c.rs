// https://atcoder.jp/contests/abc302/tasks/abc302_c

use proconio::input;
use itertools::Itertools;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    'outer: for perm in (0..N).permutations(N) {
        for i in 0..N-1 {
            let mut cnt = 0;
            for j in 0..M {
                if S[perm[i]][j] != S[perm[i+1]][j] {
                    cnt += 1;
                }
            }
            if cnt >= 2 {
                continue 'outer;
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}