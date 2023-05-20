// https://atcoder.jp/contests/abc253/tasks/abc253_c

use proconio::input;
use proconio::fastout;
use std::collections::BTreeMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut map = BTreeMap::new();
    for _ in 0..Q {
        input! {
            query: usize,
        }
        if query == 1 {
            input! {
                x: usize,
            }
            *map.entry(x).or_insert(0) += 1;
        }
        else if query == 2 {
            input! {
                x: usize,
                c: usize,
            }
            if !map.contains_key(&x) {
                continue;
            }
            let val = *map.get(&x).unwrap();
            if val <= c {
                map.remove(&x);
            }
            else {
                *map.entry(x).or_insert(0) = val - c;
            }
        }
        else {
            let mi = *map.keys().next().unwrap();
            let ma = *map.keys().last().unwrap();
            println!("{}", ma - mi);
        }
    }
}