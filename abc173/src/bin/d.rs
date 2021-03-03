use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u128; n]
    }

    a.sort_unstable_by(|a, b| b.cmp(&a));

    let mut stack = vec![0];
    let mut sum = 0;
    for i in 0..a.len() {
        sum += stack.pop().unwrap();
        stack.push(a[i]);
        if i > 0 {
            stack.push(a[i]);
        }
    }
    println!("{}", sum);
}
