// https://atcoder.jp/contests/abc231/tasks/abc231_d

use proconio::input;
use proconio::fastout;

struct UnionFind {
    par: Vec<isize>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let par = vec![-1; n];
        let sz = vec![1; n];
        Self {
            par,
            sz,
        }
    }

    pub fn root(&mut self, mut x: usize) -> usize {
        while self.par[x] != -1 {
            x = self.par[x] as usize;
        }
        return x;
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let u = self.root(u);
        let v = self.root(v);
        if u == v {
            return;
        }
        if self.sz[u] < self.sz[v] {
            self.par[u] = v as isize;
            self.sz[v] += self.sz[u];
        }
        else {
            self.par[v] = u as isize;
            self.sz[u] += self.sz[v];
        }
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        if self.root(u) == self.root(v) {
            return true;
        }
        return false;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut UF = UnionFind::new(N);
    let mut cnt = vec![0; N];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        if UF.same(A-1, B-1) {
            println!("No");
            return;
        }
        if cnt[A-1] == 2 || cnt[B-1] == 2 {
            println!("No");
            return;
        }
        UF.unite(A-1, B-1);
        cnt[A-1] += 1;
        cnt[B-1] += 1;
    }
    println!("Yes");
}