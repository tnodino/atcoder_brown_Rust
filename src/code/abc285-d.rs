// https://atcoder.jp/contests/abc285/tasks/abc285_d

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut S = Vec::new();
    let mut T = Vec::new();
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
            t: String,
        }
        S.push(s.clone());
        T.push(t.clone());
        vec.push(s);
        vec.push(t);
    }
    vec.sort();
    vec.dedup();
    let M = vec.len();
    let mut map = HashMap::new();
    for i in 0..M {
        map.insert(&vec[i], i);
    }
    let mut G = vec![Vec::new(); M];
    for i in 0..N {
        G[map[&S[i]]].push(map[&T[i]]);
    }
    let mut flg = vec![false; M];
    let mut fin = vec![false; M];
    for i in 0..M {
        if !fin[i] {
            let mut pos = i;
            let mut vec = vec![i];
            while !G[pos].is_empty() {
                let nxt = G[pos][0];
                if fin[nxt] {
                    break;
                }
                if flg[nxt] {
                    println!("No");
                    return;
                }
                flg[nxt] = true;
                pos = nxt;
                vec.push(nxt);
            }
            for j in 0..vec.len() {
                fin[vec[j]] = true;
            }
        }
    }
    println!("Yes");
}