use proconio::input;

fn main() {
    input! {
        a: usize
    }

    if a == 1 {
        println!("1 1");
    } else {
        println!("{} 2", a + 1);
    }
}
