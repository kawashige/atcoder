use proconio::input;

fn main() {
    input! {
        n: usize,
       a: [usize; n]
    }

    if a.iter().all(|x| x % 2 == 0) {
        println!("second");
    } else {
        println!("first");
    }
}
