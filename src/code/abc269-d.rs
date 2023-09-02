// https://atcoder.jp/contests/abc269/tasks/abc269_d

use proconio::input;
use proconio::fastout;

pub struct Dsu {
    n: usize,
    parent_or_size: Vec<i32>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }

    pub fn leader(&mut self, a: usize) -> usize {
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
        self.parent_or_size[a] as usize
    }

    pub fn size(&mut self, a: usize) -> usize {
        let x = self.leader(a);
        -self.parent_or_size[x] as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}

const DX: &[isize] = &[-1, 0,-1, 1, 0, 1];
const DY: &[isize] = &[-1,-1, 0, 0, 1, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut X = Vec::new();
    let mut Y = Vec::new();
    for _ in 0..N {
        input! {
            x: isize,
            y: isize,
        }
        X.push(x);
        Y.push(y);
    }
    let mut UF = Dsu::new(N);
    for i in 0..N {
        for j in i+1..N {
            for d in 0..6 {
                if X[i] + DX[d] == X[j] && Y[i] + DY[d] == Y[j] {
                    UF.merge(i, j);
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..N {
        if UF.leader(i) == i {
            ans += 1;
        }
    }
    println!("{}", ans);
}