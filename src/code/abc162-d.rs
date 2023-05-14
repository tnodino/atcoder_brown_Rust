// https://atcoder.jp/contests/abc162/tasks/abc162_d

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut cnt: [usize; 3] = [0; 3];
    for i in 0..N {
        cnt[match S[i] {
            'R' => 0,
            'G' => 1,
            _ => 2,
        }] += 1;
    }
    let mut ans = cnt[0] * cnt[1] * cnt[2];
    'outer: for i in 0..N {
        for j in i+1..N {
            let k = (j - i) + j;
            if k >= N {
                continue 'outer;
            }
            if S[i] != S[j] && S[i] != S[k] && S[j] != S[k] {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}