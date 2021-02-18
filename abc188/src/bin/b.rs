use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }

    if (0..n).map(|i| a[i] * b[i]).sum::<i32>() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
