use proconio::input;

fn main() {
    input! {
        l1: usize,
        l2: usize,
        l3: usize
    }

    if l1 == l2 {
        println!("{}", l3);
    } else if l1 == l3 {
        println!("{}", l2);
    } else {
        println!("{}", l1);
    }
}
