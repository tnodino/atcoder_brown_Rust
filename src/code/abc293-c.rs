// https://atcoder.jp/contests/abc293/tasks/abc293_c

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[allow(non_snake_case)]
fn dfs(H: &usize, W: &usize, A: &Vec<Vec<usize>>, x: usize, y: usize, set: &mut HashSet<usize>) -> usize {
    if set.contains(&A[x][y]) {
        return 0;
    }
    if x + 1 == *H && y + 1 == *W {
        return 1;
    }
    let mut ret = 0;
    set.insert(A[x][y]);
    if x + 1 < *H {
        ret += dfs(H, W, A, x + 1, y, set);
    }
    if y + 1 < *W {
        ret += dfs(H, W, A, x, y + 1, set);
    }
    set.remove(&A[x][y]);
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[usize; W]; H],
    }
    let mut set = HashSet::new();
    println!("{}", dfs(&H, &W, &A, 0, 0, &mut set));
}