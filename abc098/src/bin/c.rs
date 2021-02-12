use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        directions: Chars,
    }

    let mut wests = vec![0; n + 1];
    for i in 0..n {
        wests[i + 1] = wests[i] + if directions[i] == 'W' { 1 } else { 0 };
    }
    let mut easts = vec![0; n + 1];
    for i in (0..n).rev() {
        easts[i] = easts[i + 1] + if directions[i] == 'E' { 1 } else { 0 };
    }

    let min = (0..n).map(|i| wests[i] + easts[i + 1]).min().unwrap();
    println!("{}", min);
}
