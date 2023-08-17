// https://atcoder.jp/contests/abc264/tasks/abc264_d

use proconio::input;
use proconio::fastout;
use std::collections::{VecDeque, HashMap};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = 7;
    let mut map = HashMap::new();
    map.insert(S.clone(), 0);
    let mut que = VecDeque::new();
    que.push_back(S);
    while !que.is_empty() {
        let s = que.pop_front().unwrap();
        let cnt = map[&s];
        let mut s = s.chars().collect::<Vec<char>>();
        for i in 0..N-1 {
            s.swap(i, i+1);
            let t = s.iter().collect::<String>();
            if !map.contains_key(&t) {
                map.insert(t.clone(), cnt+1);
                que.push_back(t);
            }
            s.swap(i, i+1);
        }
    }
    println!("{}", map[&("atcoder".to_string())]);
}