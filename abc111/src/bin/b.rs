use proconio::input;

fn main() {
    input! {
        n: usize
    }

    for i in 1..10 {
        let num = i * 100 + i * 10 + i;
        if n <= num {
            println!("{}", num);
            return;
        }
    }
}
