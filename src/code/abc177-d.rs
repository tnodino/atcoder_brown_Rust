// https://atcoder.jp/contests/abc177/tasks/abc177_d

#[allow(non_snake_case)]
mod UnionFind {
    pub struct UnionFind {
        p: Vec<Option<usize>>,
        r: Vec<usize>,
        gn: usize,
    }

    impl UnionFind {
        pub fn init(N: usize) -> Self {
            let p: Vec<Option<usize>> = vec![None; N];
            let r: Vec<usize> = vec![1; N];
            let gn: usize = N;
            Self {
                p,
                r,
                gn,
            }
        }

        pub fn merge(&mut self, a: usize, b: usize) {
            let x = self.group(a);
            let y = self.group(b);
            if x == y {
                return;
            }
            self.gn -= 1;
            if self.r[x] < self.r[y] {
                self.p[x] = Some(y);
                self.r[y] += self.r[x];
            }
            else {
                self.p[y] = Some(x);
                self.r[x] += self.r[y];
            }
        }

        pub fn group(&mut self, a: usize) -> usize {
            if self.p[a] == None {
                return a;
            }
            let res = self.group(self.p[a].unwrap());
            self.p[a] = Some(res);
            return res;
        }

        pub fn size(&mut self, a: usize) -> usize {
            let res = self.group(a);
            return self.r[res];
        }
    }
}

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut UF = UnionFind::UnionFind::init(N);
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        UF.merge(A-1, B-1);
    }
    let mut ans = 0;
    for i in 0..N {
        ans = max(ans, UF.size(i));
    }
    println!("{}", ans);
}