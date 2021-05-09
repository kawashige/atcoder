use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        k: u64
    }

    if s.iter().take_while(|c| c == &&'1').count() as u64 >= k {
        println!("1");
    } else {
        println!(
            "{}",
            s.into_iter().skip_while(|c| c == &'1').next().unwrap()
        );
    }
}
