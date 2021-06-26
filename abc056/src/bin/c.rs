use proconio::input;

fn main() {
    input! {
        x :usize
    }

    let mut n = 0;
    for i in 1..=x {
        n += i;
        if n >= x {
            println!("{}", i);
            return;
        }
    }
}
