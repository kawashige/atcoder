use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u128; n]
    }

    if a.iter().any(|num| num == &0) {
        println!("0");
        return;
    }

    let mut m = 1;
    for num in a {
        m *= num;
        if m > 1_000_000_000_000_000_000 {
            println!("-1");
            return;
        }
    }
    println!("{}", m);
}
