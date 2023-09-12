// https://atcoder.jp/contests/abc259/tasks/abc259_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (S, T): (String, String),
    }
    let N = S.len();
    let M = T.len();
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    let mut cnts = Vec::new();
    let mut cnt = 1;
    let mut l = 0;
    for i in 1..N {
        if S[l] != S[i] {
            cnts.push((S[l], cnt));
            cnt = 0;
            l = i;
        }
        cnt += 1;
    }
    cnts.push((S[l], cnt));
    let mut cntt = Vec::new();
    let mut cnt = 1;
    let mut l = 0;
    for i in 1..M {
        if T[l] != T[i] {
            cntt.push((T[l], cnt));
            cnt = 0;
            l = i;
        }
        cnt += 1;
    }
    cntt.push((T[l], cnt));
    if cnts.len() != cntt.len() {
        println!("No");
        return;
    }
    let N = cnts.len();
    for i in 0..N {
        if cnts[i].0 != cntt[i].0 {
            println!("No");
            return;
        }
        if cnts[i].1 > cntt[i].1 || (cnts[i].1 == 1 && cntt[i].1 > 1) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}