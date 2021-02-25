use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i128; n]
    }

    println!("{}", x.iter().map(|n| n.abs()).sum::<i128>());
    println!(
        "{:.15}",
        (x.iter().map(|n| n * n).sum::<i128>() as f64).sqrt()
    );
    println!("{}", x.iter().map(|n| n.abs()).max().unwrap());
}
