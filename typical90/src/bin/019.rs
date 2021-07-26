use proconio::input;

pub fn recurse(s: usize, e: usize, a: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
    if s + 1 == e {
        return (a[s] - a[e]).abs();
    }

    if memo[s][e] != -1 {
        return memo[s][e];
    }

    let mut min = std::i32::MAX;
    for i in ((s + 1)..=e).step_by(2) {
        let mut sum = (a[s] - a[i]).abs();
        if s + 1 != i {
            sum += recurse(s + 1, i - 1, a, memo);
        }
        if i != e {
            sum += recurse(i + 1, e, a, memo);
        }

        min = min.min(sum);
    }

    memo[s][e] = min;
    min
}

fn main() {
    input! {
        n: usize,
        a: [i32; n * 2]
    }

    let mut memo = vec![vec![-1; n * 2]; n * 2];
    let r = recurse(0, 2 * n - 1, &a, &mut memo);
    println!("{}", r);
}
