// https://atcoder.jp/contests/abc259/tasks/abc259_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let N = S.len();
    let M = T.len();
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    let mut vecs = Vec::new();
    let mut vect = Vec::new();
    let mut cnt = 1;
    for i in 1..N {
        if S[i-1] != S[i] {
            vecs.push((S[i-1], cnt));
            cnt = 0;
        }
        cnt += 1;
    }
    vecs.push((S[N-1], cnt));
    let mut cnt = 1;
    for i in 1..M {
        if T[i-1] != T[i] {
            vect.push((T[i-1], cnt));
            cnt = 0;
        }
        cnt += 1;
    }
    vect.push((T[M-1], cnt));
    if vecs.len() != vect.len() {
        println!("No");
        return;
    }
    let N = vecs.len();
    let mut ans = "Yes";
    for i in 0..N {
        if vecs[i].0 != vect[i].0 {
            ans = "No";
        }
        if vecs[i].1 > vect[i].1 {
            ans = "No";
        }
        if vecs[i].1 < vect[i].1 && vecs[i].1 == 1 {
            ans = "No";
        }
    }
    println!("{}", ans);
}