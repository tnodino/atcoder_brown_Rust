// https://atcoder.jp/contests/arc135/tasks/arc135_a

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

const MOD: usize = 998_244_353;

fn f(x: usize, map: &mut HashMap<usize, usize>) -> usize {
    if x <= 4 {
        return x;
    }
    if map.contains_key(&x) {
        return map[&x];
    }
    let ret = f(x / 2, map) * f((x + 1) / 2, map) % MOD;
    map.insert(x, ret);
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    let mut map = HashMap::new();
    println!("{}", f(X, &mut map));
}