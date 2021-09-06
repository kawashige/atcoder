use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n]
    }

    if let Some(i) = (0..(n - 1)).find(|i| l <= a[*i] + a[i + 1]) {
        println!("Possible");
        for j in 0..i {
            println!("{}", j + 1);
        }
        for j in ((i + 1)..(n - 1)).rev() {
            println!("{}", j + 1);
        }
        println!("{}", i + 1);
    } else {
        println!("Impossible");
    }
}
