use proconio::input;

fn main() {
    input! {
        t: usize,
        case: [u64; t]
    }

    for x in case {
        if x % 2 == 0 {
            if x / 2 % 2 == 0 {
                println!("Even")
            } else {
                println!("Same")
            }
        } else {
            println!("Odd")
        }
    }
}
