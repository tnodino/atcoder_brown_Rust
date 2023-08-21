// https://atcoder.jp/contests/abc243/tasks/abc243_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut X: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut T = Vec::new();
    let mut l = 0;
    for i in 0..N {
        if l == 0 {
            T.push(S[i]);
            l += 1;
        }
        else {
            if T[l-1] != 'U' && S[i] == 'U' {
                T.pop();
                l -= 1;
            }
            else {
                T.push(S[i]);
                l += 1;
            }
        }
    }
    for i in 0..l {
        match T[i] {
            'U' => X /= 2,
            'L' => X *= 2,
            _ => X = X * 2 + 1,
        }
    }
    println!("{}", X);
}