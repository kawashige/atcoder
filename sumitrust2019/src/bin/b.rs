use proconio::input;

fn main() {
    input! {
        n: usize
    }

    for i in 1..=n {
        if i + (i * 8) / 100 == n {
            println!("{}", i);
            return;
        }
    }
    println!(":(");
}
