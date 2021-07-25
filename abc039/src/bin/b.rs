use proconio::input;

fn main() {
    input! {
        x: usize
    }

    for i in 1..=((x as f64).sqrt().sqrt() as usize) {
        if i * i * i * i == x {
            println!("{}", i);
            return;
        }
    }
}
