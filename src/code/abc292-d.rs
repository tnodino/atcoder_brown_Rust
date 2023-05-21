// https://atcoder.jp/contests/abc292/tasks/abc292_d
#[allow(non_snake_case)]
struct UnionFind {
    p: Vec<isize>,
    r: Vec<isize>,
    gn: isize,
}

#[allow(non_snake_case)]
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            p: vec![-1; n],
            r: vec![0; n],
            gn: n as isize,
        }
    }

    fn merge(&mut self, a: usize, b: usize) {
        let x = self.group(a);
        let y = self.group(b);
        if x == y {
            self.r[x] += 1;
            return;
        }
        self.gn -= 1;
        if self.r[x] < self.r[y] {
            self.p[y] += self.p[x];
            self.p[x] = y as isize;
            self.r[y] += self.r[x] + 1;
        } else {
            self.p[x] += self.p[y];
            self.p[y] = x as isize;
            self.r[x] += self.r[y] + 1;
        }
    }

    fn group(&mut self, a: usize) -> usize {
        if self.p[a] <= -1 {
            return a;
        }
        let p_a = self.p[a] as usize;
        self.p[a] = self.group(p_a) as isize;
        return self.p[a] as usize
    }
}

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut UF = UnionFind::new(N);
    for _ in 0..M {
        input! {
            u: usize,
            v: usize,
        }
        UF.merge(u - 1, v - 1);
    }
    for i in 0..N {
        let idx = UF.group(i);
        if -UF.p[idx] != UF.r[idx] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}