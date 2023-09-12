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
            q: usize,
        }
        match q {
            1 => {
                input! {
                    x: usize,
                }
                *map.entry(x).or_insert(0) += 1;
            },
            2 => {
                input! {
                    (x, c): (usize, isize),
                }
                *map.entry(x).or_insert(0) -= c;
                if map[&x] <= 0 {
                    map.remove(&x);
                }
            },
            3 => {
                let mi = *map.keys().next().unwrap();
                let ma = *map.keys().last().unwrap();
                println!("{}", ma - mi);
            },
            _ => {},
        }
    }
}