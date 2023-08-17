// https://atcoder.jp/contests/abc241/tasks/abc241_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let m = 6;
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    for i in 0..=N-m {
        for j in 0..N {
            let mut vec = Vec::new();
            for k in 0..m {
                vec.push(S[i+k][j]);
            }
            if vec.iter().filter(|&x| *x == '#').count() >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    for i in 0..N {
        for j in 0..=N-m {
            let mut vec = Vec::new();
            for k in 0..m {
                vec.push(S[i][j+k]);
            }
            if vec.iter().filter(|&x| *x == '#').count() >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    for i in 0..=N-m {
        for j in 0..=N-m {
            let mut vec = Vec::new();
            for k in 0..m {
                vec.push(S[i+k][j+k]);
            }
            if vec.iter().filter(|&x| *x == '#').count() >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    for i in m-1..N {
        for j in 0..=N-m {
            let mut vec = Vec::new();
            for k in 0..m {
                vec.push(S[i-k][j+k]);
            }
            if vec.iter().filter(|&x| *x == '#').count() >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}