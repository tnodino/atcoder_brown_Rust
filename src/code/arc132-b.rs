// https://atcoder.jp/contests/arc132/tasks/arc132_b

use proconio::input;
use proconio::fastout;
use std::collections::{VecDeque, HashMap};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let p = p.iter().map(|&x| x-1).collect::<Vec<usize>>();
    let l = p[0];
    let flg = match (p[0] + 1) % n == p[1] {
        true => 0,
        false => 1,
    };
    let mut map = HashMap::new();
    map.insert((l, flg), 0);
    let mut que = VecDeque::new();
    que.push_back((l, flg));
    while !que.is_empty() {
        let (l, flg) = que.pop_front().unwrap();
        let cnt = map[&(l, flg)];
        if flg == 0 {
            if !map.contains_key(&((l + 1) % n, 0)) {
                map.insert(((l + 1) % n, 0), cnt + 1);
                que.push_back(((l + 1) % n, 0));
            }
            if !map.contains_key(&((l + n - 1) % n, 1)) {
                map.insert(((l + n - 1) % n, 1), cnt + 1);
                que.push_back(((l + n - 1) % n, 1));
            }
        }
        else {
            if !map.contains_key(&((l + n - 1) % n, 1)) {
                map.insert(((l + n - 1) % n, 1), cnt + 1);
                que.push_back(((l + n - 1) % n, 1));
            }
            if !map.contains_key(&((l + 1) % n, 0)) {
                map.insert(((l + 1) % n, 0), cnt + 1);
                que.push_back(((l + 1) % n, 0));
            }
        }
    }
    println!("{}", map[&(0, 0)]);
}