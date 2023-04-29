// https://atcoder.jp/contests/abc143/tasks/abc143_d

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if &arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return left;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut L: [usize; N],
    }
    L.sort();
    let mut ans = 0;
    for i in 0..N {
        for j in 0..i {
            let idx = lower_bound(&L, &(L[i] + L[j]));
            ans += idx - i - 1;
        }
    }
    println!("{}", ans);
}