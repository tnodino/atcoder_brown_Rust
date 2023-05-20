// https://atcoder.jp/contests/arc106/tasks/arc106_b
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
            r: vec![1; n],
            gn: n as isize,
        }
    }

    fn merge(&mut self, a: usize, b: usize) {
        let x = self.group(a);
        let y = self.group(b);
        if x == y {
            return;
        }
        self.gn -= 1;
        if self.r[x] < self.r[y] {
            self.p[x] = y as isize;
        } else {
            self.p[y] = x as isize;
            if self.r[x] == self.r[y] {
                self.r[x] += 1;
            }
        }
    }

    fn group(&mut self, a: usize) -> usize {
        if self.p[a] == -1 {
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
        A: [isize; N],
        B: [isize; N],
    }
    let mut UF = UnionFind::new(N);
    for _ in 0..M {
        input! {
            C: usize,
            D: usize,
        }
        UF.merge(C - 1, D - 1);
    }
    let mut a = vec![0; N];
    let mut b = vec![0; N];
    for i in 0..N {
        a[UF.group(i)] += A[i];
        b[UF.group(i)] += B[i];
    }
    for i in 0..N {
        if a[i] != b[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}