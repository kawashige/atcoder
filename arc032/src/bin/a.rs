use proconio::input;

fn main() {
    input! {
        n: usize
    }

    if n == 1 {
        println!("BOWWOW");
        return;
    }

    let x = n * (n + 1) / 2;

    for i in 2..=((x as f64).sqrt() as usize) {
        if x % i == 0 {
            println!("BOWWOW");
            return;
        }
    }
    println!("WANWAN");
}
