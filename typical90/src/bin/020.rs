use proconio::input;

fn main() {
    input! {
        a: u64,
        mut b: u64,
        mut c: u64
    }

    let mut num = 1;
    while b > 0 {
        num *= c;
        b -= 1;
    }

    if a < num {
        println!("Yes");
    } else {
        println!("No");
    }
}
