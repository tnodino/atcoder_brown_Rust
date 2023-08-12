// https://atcoder.jp/contests/abc243/tasks/abc243_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            X: usize,
            Y: usize,
        }
        vec.push((Y, X));
    }
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut mapl = HashMap::new();
    let mut mapr = HashMap::new();
    for i in 0..N {
        if S[i] == 'L' {
            mapl.entry(vec[i].0).and_modify(|v| {
                if *v < vec[i].1 {
                    *v = vec[i].1;
                }
            }).or_insert(vec[i].1);
        }
        else {
            mapr.entry(vec[i].0).and_modify(|v| {
                if *v > vec[i].1 {
                    *v = vec[i].1;
                }
            }).or_insert(vec[i].1);
        }
    }
    for (k, v) in mapr.iter() {
        if !mapl.contains_key(k) {
            continue;
        }
        if v < &mapl[k] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}