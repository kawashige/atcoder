use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut count = vec![0; 9];
    for x in a {
        let i = (x / 400).min(8);
        count[i] += 1;
    }

    let sum = count[..8].iter().filter(|c| c > &&0).count();
    println!(
        "{} {}",
        if sum == 0 && count[8] > 0 { 1 } else { sum },
        sum + count[8]
    );
}
