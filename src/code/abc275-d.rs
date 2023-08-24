// https://atcoder.jp/contests/abc275/tasks/abc275_d

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

fn f(x: usize, map: &mut HashMap<usize, usize>) -> usize {
    if map.contains_key(&x) {
        return map[&x];
    }
    let ret = f(x/2, map) + f(x/3, map);
    map.insert(x, ret);
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut map = HashMap::new();
    map.insert(0, 1);
    println!("{}", f(N, &mut map));
}