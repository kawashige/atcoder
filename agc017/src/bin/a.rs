use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n]
    }

    let odd_count = a.into_iter().filter(|x| x % 2 == 1).count();
    let even_count = n - odd_count;

    let mut factorial = vec![1; odd_count + 1];
    for i in 2..factorial.len() {
        factorial[i] = factorial[i - 1] * (i as u128);
    }

    let even = 2_u128.pow(even_count as u32);
    let mut odd = 0;
    for i in (if p == 0 { 0 } else { 1 }..=odd_count).step_by(2) {
        odd += factorial[odd_count] / (factorial[odd_count - i] * factorial[i]);
    }

    println!("{}", odd * even);
}
