use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut count = vec![0; n + 1];
    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            count[j] += 1;
        }
    }

    let r = (0..=n).filter(|i| i % 2 == 1 && count[*i] == 8).count();
    println!("{}", r);
}
