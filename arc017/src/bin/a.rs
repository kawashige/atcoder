use proconio::input;

fn main() {
    input! {
        n: usize
    }

    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
